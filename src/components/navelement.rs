use yew::prelude::*;
// use yew_router::prelude::*;

#[function_component]
pub fn NavElement() -> Html {
    return html!(
        <div class={classes!("nav_element", "inside-part")}>
            <div class={classes!("nav_btn_container")}>
                <NavBtn icon="house" dest="" />
            </div>
            <div class={classes!("nav_btn_container")}>
                <NavBtn icon="rocket" dest="SpaceTrader" />
            </div>
        </div>
    );
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    icon: String,
    dest: String,
}

#[function_component]
pub fn NavBtn(props: &ButtonProps) -> Html {
    return html! {
        <a class={classes!{"button"}} href={"#/".to_owned() + &props.dest.clone()}>
            <img src={format!("{}{}{}", "media/", props.icon.clone(), ".svg")} alt={props.icon.clone()} />
        </a>
    };
}
