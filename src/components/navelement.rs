use yew::prelude::*;

#[function_component]
pub fn NavElement() -> Html {
    return html!(
        <div class={classes!("nav_element", "inside-part")}>
            <div class={classes!("nav_btn_container")} />
            <div class={classes!("nav_btn_container")} />
        </div>
    )
}


