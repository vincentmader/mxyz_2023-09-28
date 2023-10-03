use stylist::css;
use yew::prelude::*;

use crate::services::routing::Route;
mod navbar_item;
use navbar_item::MainNavbarItem;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub route: Route,
}

#[function_component]
pub fn MainNavbar(props: &Props) -> Html {
    let Props { route } = props;

    let class = css!(
        "
            --text_color: #CCCCCC;
            --section_bg_color: #222222;

            background-color: var(--section_bg_color);
            border-bottom: 1px solid var(--text_color);
            height: 3em; 

            #left {
                float: left;
            }
            #right {
                float: right;
            }
        "
    );

    html! {
        <div {class}>
                <span id={"left"}>
                    <MainNavbarItem title={"mader.xyz"} route={Route::Home} current={route.clone()}/>
                </span>
                <span id={"right"}>
                    <MainNavbarItem title={"engine"} route={Route::Engine} current={route.clone()}/>
                    <MainNavbarItem title={"blog"} route={Route::Blog} current={route.clone()}/>
                    <MainNavbarItem title={"about"} route={Route::About} current={route.clone()}/>
                </span>
        </div>
    }
}
