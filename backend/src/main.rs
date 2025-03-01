use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures::{StreamExt, TryStreamExt};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};
use uuid::Uuid;

mod game;
mod handlers;
mod state;

#[derive(Debug)]
struct AppState {
    db: sqlx::PgPool,
    game_state: Arc<Mutex<state::GameState>>,
}

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?

    let state = app_state.clone();
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(client_msg) = serde_json::from_str(&text) {
                        handlers::handle_client_message(client_msg, &state, &mut session).await;
                    }
                }
                Message::Close(reason) => {
                    info!("Client disconnected: {:?}", reason);
                    break;
                }
                _ => continue,
            }
        }
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let app_state = web::Data::new(AppState {
        db: pool,
        game_state: Arc::new(Mutex::new(state::GameState::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/ws", web::get().to(ws_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}