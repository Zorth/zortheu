use yew::prelude::*;


#[function_component]
pub fn SpaceTrader() -> Html {

    return html!{
        <div class={classes!{"st"}}>
            <div class={classes!{"st-blur"}}/>
            <div class={classes!{"st-header"}}>
            </div>
            <div class={classes!{"st-body"}}>
                <div class={classes!{"st-left-sidebar"}}>
                    <div class={classes!{"st-sidebar-comp"}}></div>
                    <div class={classes!{"st-sidebar-comp"}}></div>
                    <div class={classes!{"st-sidebar-comp"}}></div>
                </div>
                <div class={classes!{"st-main"}}>
                    <div class={classes!{"st-main-top"}}>
                    </div>
                    <div class={classes!{"st-main-bot"}}>
                    </div>
                </div>
            </div>
        </div>
    }
}
