use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let room_name = use_state(String::new);
    let rooms = use_state(Vec::new);

    // Room creation/joining logic
    let on_room_input = {
        let room_name = room_name.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<HtmlInputElement>().value();
            room_name.set(value);
        })
    };

    html! {
        <div class="landing-page">
            <div class="room-section">
                <input 
                    type="text" 
                    placeholder="Enter room name"
                    oninput={on_room_input}
                />
                <button>{"Create/Join Room"}</button>
                <div class="room-list">
                    // Room list implementation
                </div>
            </div>
        </div>
    }
}