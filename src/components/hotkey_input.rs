use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub mobile: bool,
    pub profile: Profile,
    pub override_value: String,
    pub active: bool,
    pub update_profile: Callback<Profile>,
}

#[function_component(HotkeyInput)]
pub fn hotkey_input(props: &Props) -> Html {
    let input_ref = use_node_ref();

    let _ = {
        let input_ref = input_ref.clone();
        let value = props.override_value.clone();
        let active = props.active.clone();
        use_effect(move || {
            // Auto focus input on load
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            if input.value().is_empty()
                || input.value() == "Open Settings"
                || input.value() == "Close Settings"
            {
                input.set_value(&value);
            } else if !active {
                input.set_value("");
            }

            if active {
                let _ = input.focus();
            }
        })
    };

    let handle_hotkeys = {
        let profile = props.profile.clone();
        let active = props.active.clone();
        Callback::from(move |event: KeyboardEvent| {
            // We don't handle hotkeys if not active
            if !active {
                event.prevent_default();
                return;
            }

            let input = event.target_unchecked_into::<HtmlInputElement>();
            if input.value() == "Open Settings" {
                input.set_value("");
            }

            // Find if any command is tied value
            if event.key() == "Enter" {
                let value = event.target_unchecked_into::<HtmlInputElement>().value();
                if let Some(command) = profile
                    .commands
                    .iter()
                    .filter(|command| command.hotkey == value)
                    .next()
                {
                    match &command.command_type {
                        CommandType::Empty => {}
                        CommandType::Link(link) => {
                            open_link(link, true);
                        }
                    }
                } else {
                    // No command, so search instead
                    let search_link = profile.search_template.replace("{}", &input.value());
                    open_link(&search_link, true);
                }

                input.set_value("");
            }
        })
    };

    let handle_hotkeys_highlight = {
        let profile = props.profile.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: KeyboardEvent| {
            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            profile.check_hotkey(&value);
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
