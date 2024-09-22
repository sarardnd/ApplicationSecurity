use yew::prelude::*;
use crate::lobby::Lobby;
#[function_component(App)]
pub fn app() -> Html {
    //TODO: doesn't go to the lobby if the fields are not filled
    let show_lobby = use_state(|| false);

    let onclick = {
        let show_lobby = show_lobby.clone();
        Callback::from(move |_| {
            show_lobby.set(true);
        })
    };

    if *show_lobby {
        return html! { <Lobby /> };
    }

    html! {
        <main>
            <div class="container">
                <form action="#">
                    <div class="title">{"Login"}</div>
                    <div class="input-box underline">
                        <input type="text" placeboholder="Enter your email" required=true/>
                        <div class="underline"></div>
                    </div>
                    <div class="input-box">
                        <input type="password" placeholder="Enter your password" required=true/>
                        <div class="underline"></div>
                    </div>
                    <div class="input-box button">
                        <input type="submit" {onclick} name="" value="Continue"/>
                    </div>
                </form>
            </div>
        </main>
    }
}