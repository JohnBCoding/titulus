use web_sys::HtmlButtonElement;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub mobile: bool,
    pub profile: Profile,
    pub active: bool,
    pub selected: NodeRef,
    pub update_profile: Callback<Profile>,
    pub update_suggestions: Callback<Vec<String>>,
}

#[function_component(HotkeyInput)]
pub fn hotkey_input(props: &Props) -> Html {
    let suggestion_cooldown_state = use_state(|| false);
    let clipboard = use_clipboard();
    let input_ref = use_node_ref();

    let _ = {
        let input_ref = input_ref.clone();
        let active = props.active.clone();
        use_effect(move || {
            // Auto focus input on load
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            if active {
                let _ = input.focus();
            } else {
                input.set_value("");
            }
        })
    };

    let handle_hotkeys = {
        let profile = props.profile.clone();
        let active = props.active.clone();
        let selected = props.selected.clone();
        let update_profile = props.update_profile.clone();
        let update_suggestions = props.update_suggestions.clone();
        Callback::from(move |event: KeyboardEvent| {
            // We don't handle hotkeys if not active
            if !active && event.key() != "Escape" {
                event.prevent_default();
                return;
            }

            let input = event.target_unchecked_into::<HtmlInputElement>();
            if input.value() == "Open Settings" {
                input.set_value("");
            }

            match event.key().as_str() {
                // Find if any command is tied value and execute it
                "Enter" => {
                    let value = event.target_unchecked_into::<HtmlInputElement>().value();

                    // This can be cleaned up into a func
                    let (hotkey, search, path) = if value.len() > 1 {
                        if let Some((hotkey, extra)) = value.split_once(" ") {
                            (hotkey, extra, "")
                        } else if let Some((hotkey, extra)) = value.split_once("/") {
                            (hotkey, "", extra)
                        } else {
                            (value.as_str(), "", "")
                        }
                    } else {
                        (value.as_str(), "", "")
                    };

                    // Open url if it matches url format
                    if is_url(&value) {
                        open_link(&value, true);
                    } else if let Some(command) = profile
                        .commands
                        .iter()
                        .filter(|command| command.hotkey == hotkey)
                        .next()
                    {
                        match &command.command_type {
                            CommandType::Empty => {}
                            CommandType::Link((link, search_template)) => {
                                if search.is_empty() && path.is_empty() {
                                    open_link(link, true);
                                } else if !search.is_empty() {
                                    let search_url = search_template.replace("{}", search);
                                    open_link(&search_url, true);
                                } else if !path.is_empty() {
                                    open_link(&format!("{}/{}", link, path), true);
                                }
                            }
                            CommandType::Text(text) => {
                                clipboard.write_text(text.clone());
                            }
                        }
                    } else {
                        // No command, so search instead

                        // Check if auto complete is selected
                        if let Some(selected_node) = selected.cast::<HtmlButtonElement>() {
                            let selected_value = selected_node.value();
                            let search_link =
                                profile.search_template.replace("{}", &selected_value);
                            open_link(&search_link, true);
                        } else {
                            let search_link = profile.search_template.replace("{}", &input.value());
                            open_link(&search_link, true);
                        }
                    }

                    // Reset and update profile
                    input.set_value("");
                    let mut profile = profile.clone();
                    profile.check_hotkey("");
                    update_suggestions.emit(vec![]);
                    update_profile.emit(profile);
                }
                _ => {}
            }
        })
    };

    let handle_hotkeys_highlight = {
        let suggestion_cooldown_state = suggestion_cooldown_state.clone();
        let profile = props.profile.clone();
        let update_profile = props.update_profile.clone();
        let update_suggestions = props.update_suggestions.clone();
        Callback::from(move |event: KeyboardEvent| {
            let value = event.target_unchecked_into::<HtmlInputElement>().value();

            // This can be cleaned up into a func
            let hotkey = if value.len() > 1 {
                if let Some((hotkey, _extra)) = value.split_once(" ") {
                    hotkey
                } else if let Some((hotkey, _extra)) = value.split_once("/") {
                    hotkey
                } else {
                    &value
                }
            } else {
                &value
            };

            let mut profile = profile.clone();

            // Check for hotkey, if it fails show search suggestions
            if !profile.check_hotkey(&hotkey) {
                // Otherwise it will search when cycling suggestions
                // Need to just keep track of input state and compare for change
                if event.key() == "ArrowUp"
                    || event.key() == "ArrowDown"
                    || event.key() == "ArrowRight"
                    || event.key() == "ArrowLeft"
                {
                    return;
                }

                if !*suggestion_cooldown_state {
                    let proxy_for_auto = profile.get_current_proxy();
                    let update_suggestions = update_suggestions.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let dd_uri =
                            format!("{}https://duckduckgo.com/ac/?q={}", &proxy_for_auto, value);
                        let result = Request::get(&dd_uri).send().await;

                        match result {
                            Ok(res) => {
                                if let Ok(suggestions) =
                                    res.json::<Vec<HashMap<String, String>>>().await
                                {
                                    let suggestion_vec = suggestions
                                        .iter()
                                        .map(|suggestion| {
                                            if let Some(value) = suggestion.get("phrase") {
                                                value.to_string()
                                            } else {
                                                "".to_string()
                                            }
                                        })
                                        .collect::<Vec<String>>();

                                    update_suggestions.emit(suggestion_vec);
                                }
                            }
                            Err(err) => {
                                log!(format!("{:?}", err));
                            }
                        }
                    });

                    // Cooldown suggestions
                    let suggestion_cooldown_state_move = suggestion_cooldown_state.clone();
                    Timeout::new(500, move || {
                        suggestion_cooldown_state_move.set(false);
                    })
                    .forget();

                    suggestion_cooldown_state.set(true);
                }
            } else {
                update_suggestions.emit(vec![]);
            }

            update_profile.emit(profile);
        })
    };

    let handle_hotkeys_focus = {
        let input_ref = input_ref.clone();
        let mobile = props.mobile.clone();
        let active = props.active.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();

            // Auto focus input on desktop only, doesn't work without the delay
            if !mobile && active {
                let input_ref = input_ref.clone();
                Timeout::new(1, move || {
                    let input = input_ref.cast::<HtmlInputElement>().unwrap();
                    let _ = input.focus();
                })
                .forget();
            }
        })
    };

    html! {
        <input id="hotkey-input" onkeydown={&handle_hotkeys} onkeyup={&handle_hotkeys_highlight} onblur={&handle_hotkeys_focus} ref={input_ref}/>
    }
}
