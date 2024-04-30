use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub mobile: bool,
    pub profile: Profile,
}

#[function_component(Commands)]
pub fn commands(props: &Props) -> Html {
    let command_html = {
        let mut html_vec = Vec::new();
        props
            .profile
            .commands
            .iter()
            .for_each(|command| html_vec.push(html! {<Saved command={command.clone()}/>}));

        html_vec
    };

    html! {
        <div class="command-container row flex-center-x">
            if !&props.mobile {
                <div class="col">
                    {command_html[0].clone()}
                    {command_html[1].clone()}
                    {command_html[2].clone()}
                    {command_html[3].clone()}
                </div>
                <div class="col">
                    {command_html[4].clone()}
                    {command_html[5].clone()}
                    {command_html[6].clone()}
                    {command_html[7].clone()}
                </div>
                <div class="col">
                    {command_html[8].clone()}
                    {command_html[9].clone()}
                    {command_html[10].clone()}
                    {command_html[11].clone()}
                </div>
                <div class="col">
                    {command_html[12].clone()}
                    {command_html[13].clone()}
                    {command_html[14].clone()}
                    {command_html[15].clone()}
                </div>
            } else {
                <div class="col">
                    {command_html[0].clone()}
                    {command_html[1].clone()}
                    {command_html[2].clone()}
                    {command_html[3].clone()}
                    {command_html[4].clone()}
                    {command_html[5].clone()}
                    {command_html[6].clone()}
                    {command_html[7].clone()}
                </div>
                <div class="col">
                    {command_html[8].clone()}
                    {command_html[9].clone()}
                    {command_html[10].clone()}
                    {command_html[11].clone()}
                    {command_html[12].clone()}
                    {command_html[13].clone()}
                    {command_html[14].clone()}
                    {command_html[15].clone()}
                </div>
            }
        </div>
    }
}
