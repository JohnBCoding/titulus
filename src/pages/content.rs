use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let load_state = use_state(|| true);
    let mobile_state = use_state(|| false);
    let settings_state = use_state(|| false);
    let profile_state = use_state(|| Profile::new());
    let input_override_state = use_state(|| "");

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

    let handle_on_click_settings = {
        let settings_state = settings_state.clone();
        let input_override_state = input_override_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            if !*settings_state {
                input_override_state.set("Close Settings");
            } else {
                input_override_state.set("Open Settings");
            }

            settings_state.set(!*settings_state);
        })
    };

    // Checks if main is hovered then displays the open/close settings text
    let handle_on_hover_settings = {
        let settings_state = settings_state.clone();
        let input_override_state = input_override_state.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target_unchecked_into::<HtmlDivElement>();
            log!("{} target_id: ", target.id());
            if target.id() != "" {
                if !*settings_state {
                    input_override_state.set("Open Settings");
                } else {
                    input_override_state.set("Close Settings");
                }
            } else if !input_override_state.is_empty() {
                input_override_state.set("");
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
                <HotkeyInput mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} override_value={*input_override_state} active={!*settings_state} update_profile={&handle_on_update_profile}/>
                if !*settings_state {
                    <Commands mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} />
                } else {
                    <Settings mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()} update_profile={&handle_on_update_profile} />
                }
            </div>
        </main>
    }
}
