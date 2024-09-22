use yew::prelude::*;

#[function_component(Lobby)]
pub(crate) fn lobby() -> Html {
    html! {
        <div>
            <h1>{"Lobby"}</h1>
            <p>{"Welcome to the lobby!"}</p>
        </div>
    }
}
