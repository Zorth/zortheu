use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navelement::NavElement;

use gloo::console::log;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    log!("Example console log");

    html! {
        <main>
            <NavElement />
            <BrowserRouter>

            <Switch<Route> render={switch} />
            </BrowserRouter>
            </main>
    }
}
