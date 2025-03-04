use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;
use yew::prelude::*;
use shared::{ClientMessage, ServerMessage, GamePhase, CharacterClass};
use uuid::Uuid;

mod components;
mod game_state;
mod websocket;

use components::{landing_page::LandingPage, leaderboard::Leaderboard, player_details::PlayerDetails};

#[derive(Clone, PartialEq)]
enum Route {
    Landing,
    Leaderboard,
    PlayerDetails(String),
}

#[function_component(App)]
pub fn app() -> Html {
    let current_route = use_state(|| Route::Landing);
    
    let switch_to_landing = {
        let current_route = current_route.clone();
        Callback::from(move |_| current_route.set(Route::Landing))
    };
    
    let switch_to_leaderboard = {
        let current_route = current_route.clone();
        Callback::from(move |_| current_route.set(Route::Leaderboard))
    };

    html! {
        <div class="app-container">
            <nav>
                <button onclick={switch_to_landing}>{"Rooms"}</button>
                <button onclick={switch_to_leaderboard}>{"Leaderboard"}</button>
            </nav>
            {
                match (*current_route).clone() {
                    Route::Landing => html! { <LandingPage /> },
                    Route::Leaderboard => html! { <Leaderboard /> },
                    Route::PlayerDetails(_id) => html! { <PlayerDetails /> },
                }
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}