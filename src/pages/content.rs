use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let load_state = use_state(|| true);
    let mobile_state = use_state(|| false);
    let settings_state = use_state(|| false);
    let profile_state = use_state(|| Profile::new());
    let popup_state = use_state(|| html! {<p class="popup flex-center-x">{""}</p>});

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
        move |event: KeyboardEvent| {
            if event.key() == "Escape" {
                settings_state.set(!*settings_state);
            }
        }
    });

    let handle_on_click_settings = {
        let settings_state = settings_state.clone();
        let popup_state = popup_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if !*settings_state {
                popup_state.set(html! {<p class="popup flex-center-x">{"Close Settings"}</p>});
            } else {
                popup_state.set(html! {<p class="popup flex-center-x">{"Open Settings"}</p>});
            }

            settings_state.set(!*settings_state);
        })
    };

    // Checks if main is hovered then displays the open/close settings text
    let handle_on_hover_settings = {
        let settings_state = settings_state.clone();
        let popup_state = popup_state.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlDivElement>();
            if let Some(_target_name) = target.get_attribute("name") {
                // let inner_text = format!("{}", target.inner_text());
                popup_state.set(html! {<p class="popup flex-center-x">{"test"}</p>});
            } else if target.id() != "" {
                if !*settings_state {
                    popup_state.set(html! {<p class="popup flex-center-x">{"Open Settings"}</p>});
                } else {
                    popup_state.set(html! {<p class="popup flex-center-x">{"Close Settings"}</p>});
                }
            } else {
                popup_state.set(html! {<p class="popup flex-center-x">{""}</p>});
            }
        })
    };

    let handle_on_update_profile = {
        let profile_state = profile_state.clone();
        Callback::from(move |new_profile: Profile| {
            profile_state.set(new_profile);
        })
    };

    html! {
        <main class="col expand-x expand-y fade-in">
            <div id="main" class="main-container col expand-x expand-y" onclick={&handle_on_click_settings} onmouseover={&handle_on_hover_settings}>
                <div class="content-container flex-center-x flex-center-y">
                    <HotkeyInput mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} active={!*settings_state} update_profile={&handle_on_update_profile} />
                    <div class="row">
                        {popup_state.deref().clone()}
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
