use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "This comes from app.rs" }</h1>
        </main>
    }
}
