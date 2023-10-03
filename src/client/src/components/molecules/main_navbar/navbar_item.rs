use stylist::css;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::services::routing::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub route: Route,
    pub current: Route,
}

#[function_component]
pub fn MainNavbarItem(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let route = props.route;
    let onclick = Callback::from(move |_| navigator.push(&route));

    let class = css!(
        r"
            button {
                background-color: var(--section_bg_color);
                border: none;
                border-radius: 5px;
                color: var(--text_color);
                font-weight: bold;
                font-size: 1em;
                margin: .5em;
                padding: .5em;
            }
            button:hover {
                color: white;
                cursor: pointer;
            }
        "
    );

    html! {
        <span {class}>
            <button {onclick}>
                { &props.title }
            </button>
        </span>
    }
}
