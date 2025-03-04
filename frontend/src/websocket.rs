use web_sys::WebSocket;
use yew::Callback;

pub struct WebsocketService {
    ws: WebSocket,
    _on_message: Callback<String>, // Store the callback for future use
}

impl WebsocketService {
    pub fn new(url: &str, on_message: Callback<String>) -> Self {
        let ws = WebSocket::new(url).expect("Failed to create WebSocket");
        Self { 
            ws,
            _on_message: on_message,
        }
    }
}