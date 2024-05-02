use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let load_state = use_state(|| true);
    let mobile_state = use_state(|| false);
    let settings_state = use_state(|| false);
    let profile_state = use_state(|| Profile::new());
    let suggestions_state = use_state(|| vec![]);
    let popup_selected_state = use_state(|| 0);
    let selected_ref = use_node_ref();

    if *load_state {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap();
        if width.as_f64().unwrap() <= 1024.0 {
            mobile_state.set(true);
        } else {
            mobile_state.set(false);
        }

        profile_state.set(load());
        load_state.set(false);
    }

    // Handle global hotkeys (those that are used on main screen and settings screen)
    use_event_with_window("keydown", {
        let settings_state = settings_state.clone();
        let suggestions_state = suggestions_state.clone();
        let popup_selected_state = popup_selected_state.clone();
        move |event: KeyboardEvent| match event.key().as_str() {
            "Escape" => {
                settings_state.set(!*settings_state);
            }
            "ArrowUp" => {
                if suggestions_state.is_empty() {
                    return;
                }

                let mut popup_selected = popup_selected_state.deref().clone();
                if popup_selected > 0 {
                    popup_selected -= 1;
                } else {
                    popup_selected = min(3, suggestions_state.len() - 1);
                }

                popup_selected_state.set(popup_selected);
            }
            "ArrowDown" => {
                if suggestions_state.is_empty() {
                    return;
                }

                let mut popup_selected = popup_selected_state.deref().clone();
                if popup_selected < 3 {
                    popup_selected += 1;
                } else {
                    popup_selected = 0;
                }

                popup_selected_state.set(popup_selected);
            }
            _ => {}
        }
    });

    let handle_on_click_settings = {
        let settings_state = settings_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            settings_state.set(!*settings_state);
        })
    };

    let handle_on_update_profile = {
        let profile_state = profile_state.clone();
        Callback::from(move |new_profile: Profile| {
            profile_state.set(new_profile);
        })
    };

    let handle_on_update_suggestions = {
        let suggestions_state = suggestions_state.clone();
        let popup_selected_state = popup_selected_state.clone();
        Callback::from(move |new_suggestions: Vec<String>| {
            if new_suggestions.is_empty() {
                popup_selected_state.set(0);
            }

            suggestions_state.set(new_suggestions);
        })
    };

    let popup_html = suggestions_state
        .iter()
        .enumerate()
        .map(|(index, suggestion)| {
            if index > 2 {
                return html! {};
            }
            if index+1 == *popup_selected_state {
                html! {
                    <button value={format!("{}", suggestion)} class={"popup expand-x selected"} ref={&selected_ref}>{suggestion}</button>
                }
            } else {
                html! {
                    <button value={format!("{}", suggestion)} class={"popup expand-x"}>{suggestion}</button>
                }
            }
        })
        .collect::<Html>();

    html! {
        <main class="col expand-x expand-y fade-in">
            <div id="main" class="main-container col expand-x expand-y">
                <div class="content-container flex-center-x flex-center-y">
                    <HotkeyInput mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} active={!*settings_state} selected={selected_ref} update_profile={&handle_on_update_profile} update_suggestions={&handle_on_update_suggestions}/>
                    <div class="popup-container row">
                        if suggestions_state.is_empty() {
                            <button class={"popup expand-x"} onclick={&handle_on_click_settings}>{format!("{}", if !*settings_state{"Open Settings"} else{"Close Settings"})}</button>
                        } else {
                            {popup_html}
                        }
                    </div>
                    if !*settings_state {
                        <Commands mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} />
                    } else {
                        <Settings mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} update_profile={&handle_on_update_profile} />
                    }
                </div>
            </div>
        </main>
    }
}
