use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
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
                        <input type="submit" name="" value="Continue"/>
                    </div>
                </form>
            </div>
        </main>
    }
}
