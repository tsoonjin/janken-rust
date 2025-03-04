use yew::prelude::*;

#[function_component(PlayerDetails)]
pub fn player_details() -> Html {
    let matches = use_state(Vec::new);
    let current_tab = use_state(|| Tab::History);

    html! {
        <div class="player-details">
            <div class="tabs">
                <button>{"Match History"}</button>
                <button>{"Head to Head"}</button>
            </div>
            <div class="content">
                // Match history or head-to-head content
            </div>
        </div>
    }
}