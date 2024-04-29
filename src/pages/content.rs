use crate::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let load_state = use_state(|| true);
    let mobile_state = use_state(|| false);
    let profile_state = use_state(|| Profile::new());
    let settings_state = use_state(|| true);

    if *load_state {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap();
        if width.as_f64().unwrap() <= 1024.0 {
            mobile_state.set(true);
        } else {
            mobile_state.set(false);
        }

        load_state.set(false);
    }

    let handle_on_click_settings = {
        let settings_state = settings_state.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            settings_state.set(!*settings_state);
        })
    };

    let handle_on_click_container = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
        })
    };

    let command_options_html = profile_state
        .commands
        .iter()
        .enumerate()
        .map(|(index, command)| {
            html! {
                <option value={format!("{}", index)}>{format!("{} {}", index+1, command.name)}</option>
            }
        })
        .collect::<Html>();

    html! {
        <main class="col expand-x expand-y fade-in">
            <div class="main-container col expand-x expand-y" onclick={&handle_on_click_settings}>
                if !*settings_state {
                    <Commands mobile={mobile_state.deref().clone()} profile={profile_state.deref().clone()}/>
                } else {
                    <div class="settings-container col flex-center-x flex-center-y" onclick={&handle_on_click_container}>
                        <select>
                            {command_options_html}
                        </select>
                        <div class="row">
                            <input placeholder="Name" maxlength=24/>
                            <select class="expand-x">
                                <option value="link">{"Link"}</option>
                            </select>
                        </div>
                        <input placeholder="Value"/>
                    </div>
                }
            </div>
        </main>
    }
}
