use stylist::css;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::services::routing::Route;

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let class = css!(
        "
            padding-left: var(--main_padding_horizontal);
            padding-right: var(--main_padding_horizontal);
        "
    );

    let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div {class}>
            <h1>{ "About" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
            <ButtonClickCounter />
        </div>
    }
}

#[function_component]
fn ButtonClickCounter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
