use actix_ws::Session;
use shared::ClientMessage;
use crate::AppState;

pub async fn handle_client_message(
    msg: ClientMessage,
    state: &AppState,
    session: &mut Session,
) -> Result<(), Box<dyn std::error::Error>> {
    match msg {
        ClientMessage::CreateRoom { player_id } => {
            // Implementation
            let _ = (state, session, player_id); // Temporarily silence warnings
            Ok(())
        }
        _ => Ok(()),
    }
}