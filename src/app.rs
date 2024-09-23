use yew::prelude::*;
use crate::lobby::Lobby;
use web_sys::HtmlFormElement;

#[function_component(App)]
pub fn app() -> Html {
    let show_lobby = use_state(|| false);

    let onsubmit = {
        let show_lobby = show_lobby.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form = e.target_unchecked_into::<HtmlFormElement>();
            if form.check_validity() {
                show_lobby.set(true);
            }
        })
    };

    if *show_lobby {
        return html! { <Lobby /> };
    }

    html! {
        <main>
            <div class="container">
                <form {onsubmit}>
                    <div class="title">{"Login"}</div>
                    <div class="input-box underline">
                        <input type="text" placeholder="Enter your email" required=true/>
                        <div class="underline"></div>
                    </div>
                    <div class="input-box">
                        <input type="password" placeholder="Enter your password" required=true/>
                        <div class="underline"></div>
                    </div>
                    <div class="input-box button">
                        <input type="submit" value="Continue"/>
                    </div>
                </form>
            </div>
        </main>
    }
}