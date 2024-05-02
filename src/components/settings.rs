use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub mobile: bool,
    pub profile: Profile,
    pub update_profile: Callback<Profile>,
}

#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let command_index_state = use_state(|| 0);
    let select_ref = use_node_ref();
    let command_type_ref = use_node_ref();

    let _ = {
        let select_ref = select_ref.clone();
        use_effect(move || {
            let select = select_ref.cast::<HtmlDivElement>().unwrap();
            let _ = select.focus();
        })
    };

    let handle_on_click_container = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            event.stop_propagation();
        })
    };

    let handle_on_change_command = {
        let command_index_state = command_index_state.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event
                .target_unchecked_into::<HtmlSelectElement>()
                .value()
                .parse::<usize>()
                .unwrap();

            command_index_state.set(value);
        })
    };

    let handle_on_change_name = {
        let profile = props.profile.clone();
        let command_index_state = command_index_state.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            profile.commands[*command_index_state].name = value;

            save(&profile);
            update_profile.emit(profile);
        })
    };

    let handle_on_change_search = {
        let profile = props.profile.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            profile.search_template = value;

            save(&profile);
            update_profile.emit(profile);
        })
    };

    let handle_on_change_value = {
        let profile = props.profile.clone();
        let command_index_state = command_index_state.clone();
        let update_profile = props.update_profile.clone();
        let command_type_ref = command_type_ref.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            let command_type_value = command_type_ref
                .cast::<HtmlSelectElement>()
                .unwrap()
                .value();
            let command_type = match command_type_value.as_str() {
                "link" => CommandType::Link(value),
                _ => CommandType::Empty,
            };
            profile.commands[*command_index_state].command_type = command_type;

            save(&profile);
            update_profile.emit(profile);
        })
    };

    let handle_hotkey_key_press = {
        let profile = props.profile.clone();
        let command_index_state = command_index_state.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: KeyboardEvent| {
            event.prevent_default();

            if event.key() == "Enter" {
                return;
            }

            let input = event.target_unchecked_into::<HtmlInputElement>();
            let mut key = event.key();
            if event.key() == " " {
                key = "_".to_string();
            }

            input.set_value(&key);

            let mut profile = profile.clone();
            profile.commands[*command_index_state].hotkey = key;
            save(&profile);
            update_profile.emit(profile);
        })
    };

    let command_options_html = props.profile
        .commands
        .iter()
        .enumerate()
        .map(|(index, command)| {
            html! {
                <option value={format!("{}", index)} ~selected={if index==*command_index_state{true} else {false}}>{format!("{} {}", index+1, command.name)}</option>
            }
        })
        .collect::<Html>();

    html! {
        <div class="settings-container col flex-center-x" onclick={&handle_on_click_container} >
            <select onchange={&handle_on_change_command} ref={select_ref}>
                {command_options_html}
            </select>
            <div class="row">
                <input value={format!("{}", &props.profile.commands[*command_index_state].name)} placeholder="Name" maxlength=24 onchange={&handle_on_change_name}/>
                <select class="expand-x" ref={command_type_ref}>
                    <option value="link">{"Link"}</option>
                </select>
            </div>
            <div class="row">
                <input
                    value={
                        match &props.profile.commands[*command_index_state].command_type {
                            CommandType::Empty => {
                                "".to_string()
                            }
                            CommandType::Link(link) => {
                                link.clone()
                            }
                        }
                    }
                    class="expand-x" placeholder="Value"
                    onchange={&handle_on_change_value}
                />
                <input value={format!("{}", &props.profile.commands[*command_index_state].hotkey)} placeholder="Hotkey" onkeypress={&handle_hotkey_key_press} />
            </div>
            <input value={format!("{}", &props.profile.search_template)} class="flex-end-y" placeholder="Search Template" onchange={&handle_on_change_search} />
        </div>
    }
}
