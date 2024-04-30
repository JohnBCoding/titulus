#![allow(non_camel_case_types)]
mod components;
mod models;
mod pages;
mod utils;
mod prelude {
    pub use crate::components::*;
    pub use crate::models::*;
    pub use crate::pages::*;
    pub use crate::utils::*;
    pub use gloo_console::log;
    pub use gloo_timers::callback::Timeout;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;
    pub use std::ops::Deref;
    pub use web_sys::{HtmlButtonElement, HtmlDivElement, HtmlInputElement, HtmlSelectElement};
    pub use yew::prelude::*;
    pub use yew_router::prelude::*;
}

use prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Content,
}

fn match_route(route: Route) -> Html {
    match route {
        Route::Content => html!(<Content/>),
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={match_route} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
