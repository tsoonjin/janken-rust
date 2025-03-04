use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_ws::Message;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use tracing::info;
use uuid::Uuid;

mod game;
mod handlers;
mod state;

pub struct AppState {
    game_state: Arc<Mutex<state::GameState>>,
    db_pool: sqlx::PgPool,
}

async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    let state = app_state.clone();
    
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    if let Ok(client_msg) = serde_json::from_str(&text) {
                        handlers::handle_client_message(client_msg, &state, &mut session).await;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    Ok(response)
}

#[derive(serde::Serialize)]
struct Room {
    id: Uuid,
    name: String,
    player_count: i32,
}

async fn get_rooms() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Vec::<Room>::new()))
}

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    page: Option<i32>,
    limit: Option<i32>,
}

#[derive(Debug, Serialize)]
struct LeaderboardEntry {
    player_id: Uuid,
    username: String,
    wins: i32,
    losses: i32,
    draws: i32,
    matches: i32,
}

async fn get_leaderboard(
    app_state: web::Data<AppState>,
    query: web::Query<LeaderboardQuery>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    // Temporary empty response
    let entries: Vec<LeaderboardEntry> = Vec::new();
    Ok(HttpResponse::Ok().json(entries))
}

#[derive(serde::Serialize)]
struct PlayerDetails {
    id: Uuid,
    username: String,
    stats: PlayerStats,
}

#[derive(serde::Serialize)]
struct PlayerStats {
    wins: i32,
    losses: i32,
    draws: i32,
    matches: i32,
}

async fn get_player_details(_player_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(PlayerDetails {
        id: Uuid::new_v4(),
        username: String::new(),
        stats: PlayerStats {
            wins: 0,
            losses: 0,
            draws: 0,
            matches: 0,
        },
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app_state = web::Data::new(AppState {
        game_state: Arc::new(Mutex::new(state::GameState::new())),
        db_pool: pool,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/ws", web::get().to(ws_handler))
            .route("/api/rooms", web::get().to(get_rooms))
            .route("/api/leaderboard", web::get().to(get_leaderboard))
            .route("/api/players/{id}", web::get().to(get_player_details))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}