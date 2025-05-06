use yew::prelude::*;

use gloo::{console::log, storage::LocalStorage};
use gloo_storage::Storage;

use wasm_cookies::*;

#[function_component]
pub fn SpaceTrader() -> Html {
    let apikey_option: Option<String> = get_raw("STAPI");
    match apikey_option {
        None => log!("no API key"),
        Some(key) => log!(key),
    }
    let  cookie_opt = CookieOptions::default().expires_at_date("Mon, 14 Jun 2117 07:00:00 GMT");
    // set_raw("STAPI", "othertest", &cookie_opt);

    return html!{
        <div class={classes!{"st"}}>
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
