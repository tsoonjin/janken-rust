use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;
use yew::prelude::*;
use shared::{ClientMessage, ServerMessage, GamePhase, CharacterClass};
use uuid::Uuid;

mod components;
mod game_state;
mod websocket;

#[function_component(App)]
pub fn app() -> Html {
    let ws = use_state(|| None::<WebSocket>);
    let game_state = use_state(|| game_state::GameState::new());
    let player_id = use_state(|| Uuid::new_v4());

    let onclick = {
        let ws = ws.clone();
        let player_id = player_id.clone();
        Callback::from(move |_| {
            if ws.is_none() {
                let socket = WebSocket::new("ws://localhost:8080/ws").unwrap();
                let msg = ClientMessage::CreateRoom {
                    player_id: *player_id,
                };
                websocket::send_message(&socket, msg);
                ws.set(Some(socket));
            }
        })
    };

    html! {
        <div class="game-container">
            <h1>{"Jankenryusagi"}</h1>
            if ws.is_none() {
                <button {onclick}>{"Create Game Room"}</button>
            } else {
                <div class="game-board">
                    <components::GameBoard
                        game_state={(*game_state).clone()}
                        player_id={*player_id}
                        ws={(*ws).clone().unwrap()}
                    />
                </div>
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