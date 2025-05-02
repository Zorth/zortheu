use yew::prelude::*;


#[function_component]
pub fn Background() -> Html {
    html!{
        <div class={classes!("background")}>
            <div class={classes!("bg-blurry-1")} />
            <div class={classes!("bg-blurry-2")} />
        </div>
    }
}

