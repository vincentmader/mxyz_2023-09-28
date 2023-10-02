use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <h1>{ "404 - page not found :(" }</h1>
    }
}
