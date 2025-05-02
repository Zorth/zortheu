use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::background::Background;
use crate::components::navelement::NavElement;
use crate::pages::home::HomePage;
use crate::pages::space_trade::SpaceTrader; 

use gloo::console::log;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/SpaceTrader")]
    SpaceTrader,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<HomePage/>},
        Route::SpaceTrader => html! {<SpaceTrader/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    log!("Example console log");

    html! {
        <main>
            <div class={classes!{"foreground"}}>
                <NavElement />
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </div>
            <Background />
        </main>
    }
}
