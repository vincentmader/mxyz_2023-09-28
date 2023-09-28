use yew::prelude::*;
use yew_router::prelude::*;

use crate::screens::about::About;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! {
            <About />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
