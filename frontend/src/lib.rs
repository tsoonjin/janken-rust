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

#[function_component(App)]
pub fn app() -> Html {
    let current_route = use_state(|| Route::Landing);

    html! {
        <div class="app-container">
            <nav>
                <button onclick={/* switch to landing */}>{"Rooms"}</button>
                <button onclick={/* switch to leaderboard */}>{"Leaderboard"}</button>
            </nav>
            {
                match *current_route {
                    Route::Landing => html! { <LandingPage /> },
                    Route::Leaderboard => html! { <Leaderboard /> },
                    Route::PlayerDetails(id) => html! { <PlayerDetails player_id={id} /> },
                    Route::Game => html! { /* existing game component */ },
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