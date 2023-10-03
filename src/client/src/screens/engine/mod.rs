use stylist::css;
use yew::prelude::*;

#[function_component]
pub fn Engine() -> Html {
    let class = css!(
        "
            padding-left: var(--main_padding_horizontal);
            padding-right: var(--main_padding_horizontal);
        "
    );

    html! {
        <div {class}>
            <h1>{ "Engine" }</h1>
        </div>
    }
}
