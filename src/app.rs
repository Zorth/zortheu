use gloo::console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {

    log!("Example console log");

    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
        </main>
    }
}
