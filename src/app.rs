use yew::prelude::*;

use crate::components::navelement::NavElement;

use gloo::console::log;

#[function_component]
pub fn App() -> Html {

    log!("Example console log");

    html! {
        <main>
            <h1>{ "Hello World!!" }</h1>
            <NavElement />
        </main>
    }
}
