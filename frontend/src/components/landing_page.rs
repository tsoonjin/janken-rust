use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="landing-page">
            <h1>{"Welcome to Janken Ryusagi"}</h1>
            // Room list will go here
        </div>
    }
}