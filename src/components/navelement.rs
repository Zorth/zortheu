use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn NavElement() -> Html {
    return html!(
        <div class={classes!("nav_element", "inside-part")}>
            <div class={classes!("nav_btn_container")}>
                <NavBtn />
            </div>
            <div class={classes!("nav_btn_container")}>
                <NavBtn />
            </div>
        </div>
    );
}

#[function_component]
pub fn NavBtn() -> Html {
    return html! {
        <a class={classes!{"button"}} href="/#/SpaceTrader/">{"testbtn"}</a>
    };
}
