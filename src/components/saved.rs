use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub command: Command,
}

#[function_component(Saved)]
pub fn saved(props: &Props) -> Html {
    let handle_on_click = {
        let command = props.command.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();

            match &command.command_type {
                CommandType::Empty => {}
                CommandType::Link((link, _)) => {
                    // Open link in new window
                    let _window = web_sys::window()
                        .unwrap()
                        .window()
                        .open_with_url_and_target(link, "_blank");
                }
            }
        })
    };

    html! {
        <button name="saved-command" class={if props.command.highlight {"highlight"} else {""}} onclick={&handle_on_click}><span>{format!("<{}>", &props.command.hotkey)}</span>{&props.command.name}</button>
    }
}
