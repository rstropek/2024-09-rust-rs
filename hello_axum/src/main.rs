use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, http::StatusCode, routing::get, serve, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::RwLock};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProgrammingLanguage {
    id: Uuid,
    name: String,
    inventor: Option<String>,
    how_good_is_it: u8, // 0... really bad, 10... really good
}

type LanguageDatabase = Arc<RwLock<HashMap<Uuid, ProgrammingLanguage>>>;

#[tokio::main]
async fn main() {
    // Shared state
    let language_db = LanguageDatabase::default();

    // Add Rust to the database
    let rust = ProgrammingLanguage {
        id: Uuid::new_v4(),
        name: "Rust".to_string(),
        inventor: Some("Graydon Hoare".to_string()),
        how_good_is_it: 10,
    };
    language_db.write().await.insert(rust.id, rust);

    let javascript = ProgrammingLanguage {
        id: Uuid::new_v4(),
        name: "JavaScript".to_string(),
        inventor: Some("Brendan Eich".to_string()),
        how_good_is_it: 8,
    };
    language_db.write().await.insert(javascript.id, javascript);

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/languages", get(get_languages))
        .route("/languages/:id", get(get_language))
        .with_state(language_db);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    serve(listener, app).await.unwrap();
}

async fn ping() -> &'static str {
    "pong"
}

async fn get_languages(State(db): State<LanguageDatabase>) -> Json<Vec<ProgrammingLanguage>> {
    let languages = db.read().await.values().cloned().collect::<Vec<_>>();
    Json(languages)
}

async fn get_language(Path(id): Path<Uuid>, State(db): State<LanguageDatabase>) -> Result<Json<ProgrammingLanguage>, StatusCode> {
    let language = db.read().await.get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(language))
}
