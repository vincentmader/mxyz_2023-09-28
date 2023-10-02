use yew::prelude::*;
use yew_router::prelude::*;

use crate::screens::about::About;
use crate::screens::blog::Blog;
use crate::screens::engine::Engine;
use crate::screens::home::Home;
use crate::screens::not_found::NotFound;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/engine")]
    Engine,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Engine => html! { <Engine /> },
        Route::Blog => html! { <Blog /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
