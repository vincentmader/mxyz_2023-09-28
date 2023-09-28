use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use client::services::routing;
use client::services::routing::Route;

fn main() {
    yew::Renderer::<Main>::new().render();
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={routing::switch} />
        </BrowserRouter>
    }
}
