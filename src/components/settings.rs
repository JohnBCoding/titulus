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
    let command_type_state = use_state(|| "".to_string());
    let reader_state = use_state(|| None);
    let set_focus_state = use_state(|| true);
    let select_ref = use_node_ref();
    let import_ref = use_node_ref();

    let _ = {
        let select_ref = select_ref.clone();
        use_effect(move || {
            // Timer needed here because of no cooldown on global hotkeys which is causing multi re-renders
            if *set_focus_state {
                Timeout::new(5, move || {
                    let select = select_ref.cast::<HtmlDivElement>().unwrap();
                    let _ = select.focus();
                    set_focus_state.set(false);
                })
                .forget();
            }
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
        let command_index_state = command_index_state.clone();
        let profile = props.profile.clone();
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

    let handle_on_change_type = {
        let command_type_state = command_type_state.clone();
        let command_index_state = command_index_state.clone();
        let profile = props.profile.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlSelectElement>().value();
            let mut profile = profile.clone();

            let command_type = match value.as_str() {
                "link" => CommandType::Link(("".to_string(), "".to_string())),
                "text" => CommandType::Text("".to_string()),
                _ => CommandType::Empty,
            };

            profile.commands[*command_index_state].command_type = command_type;

            save(&profile);
            update_profile.emit(profile);
            command_type_state.set(value);
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
        let command_type_state = command_type_state.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            let old_search = match &profile.commands[*command_index_state].command_type {
                CommandType::Link((_, search)) => search,
                _ => "",
            };
            let command_type_value = command_type_state.deref().clone();
            let command_type = match command_type_value.as_str() {
                "link" => CommandType::Link((value, old_search.to_string())),
                "text" => CommandType::Text(value),
                _ => CommandType::Empty,
            };
            profile.commands[*command_index_state].command_type = command_type;

            save(&profile);
            update_profile.emit(profile);
        })
    };

    let handle_on_change_link_search = {
        let profile = props.profile.clone();
        let command_index_state = command_index_state.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            event.prevent_default();

            let value = event.target_unchecked_into::<HtmlInputElement>().value();
            let mut profile = profile.clone();
            let old_url = match &profile.commands[*command_index_state].command_type {
                CommandType::Link((url, _)) => url,
                _ => "",
            };

            let command_type = CommandType::Link((old_url.to_string(), value));
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

    let handle_import_on_click = {
        let import_ref = import_ref.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            let import = import_ref.cast::<HtmlInputElement>().unwrap();
            import.click();
        })
    };

    let handle_import_finish = {
        let reader_state = reader_state.clone();
        let update_profile = props.update_profile.clone();
        move |result: Result<String, FileReadError>| {
            if let Ok(result_str) = result {
                if let Ok(profile) = serde_json::from_str::<Profile>(&result_str) {
                    save(&profile);
                    update_profile.emit(profile);
                } else {
                    log!("Error importing profile.");
                }
            }

            reader_state.set(None);
        }
    };

    let handle_import = {
        let reader_state = reader_state.clone();
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            let files = input.files().unwrap();
            let gloo_file = File::from(files.get(0).unwrap());
            let reader = { read_as_text(&gloo_file, handle_import_finish.clone()) };
            reader_state.set(Some(reader));
        })
    };

    let handle_on_change_theme = {
        let profile = props.profile.clone();
        let update_profile = props.update_profile.clone();
        Callback::from(move |event: Event| {
            let value = event.target_unchecked_into::<HtmlSelectElement>().value();
            let index = value.parse::<usize>().unwrap_or_default();
            let mut profile = profile.clone();
            let (theme_name, _) = profile.update_theme(index);
            save(&profile);
            update_data_theme(&theme_name);
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

    let theme_options_html = props
        .profile
        .themes
        .iter()
        .enumerate()
        .map(|(index, theme)| {
            html! {
                <option value={format!("{}", index)} ~selected={index == props.profile.current_theme}>{&theme.1}</option>
            }
        })
        .collect::<Html>();

    let export_str = format!("{}", serde_json::to_string(&props.profile).unwrap());
    let export_str_url_encoded = encode(&export_str);

    html! {
        <div class="settings-container col flex-center-x" >
            <select onchange={&handle_on_change_command} ref={select_ref}>
                {command_options_html}
            </select>
            <div class="row mobile-col">
                <input value={format!("{}", &props.profile.commands[*command_index_state].name)} class="expand-x" placeholder="Name" maxlength=24 onchange={&handle_on_change_name}/>
                <select class="expand-x" onchange={&handle_on_change_type}>
                    <option value="link">{"Link"}</option>
                    <option value="text">{"Text"}</option>
                </select>
                <input value={format!("{}", &props.profile.commands[*command_index_state].hotkey)} placeholder="Hotkey" onkeypress={&handle_hotkey_key_press} />
            </div>
            <div class="row mobile-col">
                {
                    match &props.profile.commands[*command_index_state].command_type {
                        CommandType::Empty => {
                            html! {
                                <>
                                    <input value={""} class="expand-x" placeholder="URL" onchange={&handle_on_change_value} />
                                    <input value="" class="expand-x" placeholder="Search Template" onchange={&handle_on_change_link_search} />
                                </>
                            }
                        }
                        CommandType::Link((url, search)) => {
                            html! {
                                <>
                                    <input value={url.clone()} class="expand-x" placeholder="URL" onchange={&handle_on_change_value} />
                                    <input value={search.clone()} class="expand-x" placeholder="Search Template" onchange={&handle_on_change_link_search} />
                                </>
                            }
                        },
                        CommandType::Text(text) => {
                            html!{
                                <>
                                    <input value={text.clone()} class="expand-x" placeholder="Value" onchange={&handle_on_change_value} />
                                </>
                            }
                        }
                    }
                }
            </div>
            <select class="flex-end-y" onchange={&handle_on_change_theme}>
                {theme_options_html}
            </select>
            <input value={format!("{}", &props.profile.search_template)} placeholder="Search Template" onchange={&handle_on_change_search} />
            <div class="row">
                <button class="settings-button" onclick={&handle_import_on_click}>{"Import Profile"}</button>
                <input class="settings-button" type="file" onchange={&handle_import} ref={import_ref}/>
                <a class="settings-button" download={"profile.json"} href={format!("data:text/json;charset=utf-8,{}", export_str_url_encoded)}>
                        {"Export Profile"}
                </a>
            </div>
        </div>
    }
}
