use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let load_state = use_state(|| true);
    let mobile_state = use_state(|| false);
    let settings_state = use_state(|| false);
    let profile_state = use_state(|| Profile::new());
    let input_ref = use_node_ref();

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

    let _ = {
        let settings_state = settings_state.clone();
        let input_ref = input_ref.clone();
        use_effect(move || {
            if !*settings_state {
                // Auto focus input on load
                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                let _ = input.focus();
            }
        })
    };

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

    let handle_hotkeys = {
        let profile_state = profile_state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            // Find if any command is tied value
            if event.key() == "Enter" {
                let value = event.target_unchecked_into::<HtmlInputElement>().value();
                if let Some(command) = profile_state
                    .commands
                    .iter()
                    .filter(|command| command.hotkey == value)
                    .next()
                {
                    match &command.command_type {
                        CommandType::Empty => {}
                        CommandType::Link(link) => {
                            // Open link in new window
                            let _window = web_sys::window()
                                .unwrap()
                                .window()
                                .open_with_url_and_target(link, "_blank");
                        }
                    }
                }

                let input = input_ref.cast::<HtmlInputElement>().unwrap();
                input.set_value("");
            }
        })
    };

    let handle_focus_hotkeys = {
        let mobile_state = mobile_state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();

            // Auto focus input on desktop only, doesn't work without the delay
            if !*mobile_state {
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
        <main class="col expand-x expand-y fade-in">
            <div class="main-container col expand-x expand-y" onclick={&handle_on_click_settings}>
                if !*settings_state {
                    <input id="hotkey-input" class="flex-center-x" onkeydown={&handle_hotkeys} onblur={&handle_focus_hotkeys} ref={input_ref}/>
                    <Commands mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} />
                } else {
                    <input id="hotkey-input" class="flex-center-x hide" onkeydown={&handle_hotkeys} onblur={&handle_focus_hotkeys} ref={input_ref}/>
                    <Settings mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} update_profile={&handle_on_update_profile} />
                }
            </div>
        </main>
    }
}
