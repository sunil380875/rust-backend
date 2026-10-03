mod config;
mod db;
mod error;
mod middleware;
mod modules;
mod routes;
mod state;

use std::net::SocketAddr;

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::get,
    Json, Router,
};
use bson::doc;
use mongodb::IndexModel;
use serde_json::json;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ── 1. Load .env ──────────────────────────────────────────────────────────
    dotenvy::dotenv().ok();

    // ── 2. Tracing ────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rust_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── 3. Config ─────────────────────────────────────────────────────────────
    let config = Config::from_env();
    let port = config.port;
    let upload_dir = config.upload_dir.clone();
    let max_upload_bytes = config.max_upload_mb as usize * 1024 * 1024;
    info!("Configuration loaded (port={}, db={})", port, config.db_name);

    // ── 4. Ensure upload directory exists ─────────────────────────────────────
    tokio::fs::create_dir_all(&upload_dir).await?;

    // ── 5. Database ───────────────────────────────────────────────────────────
    let db = db::init_db(&config.mongo_uri, &config.db_name).await?;

    // Ensure unique index on users.email at startup
    let users_col = db.collection::<bson::Document>("users");
    users_col
        .create_index(
            IndexModel::builder()
                .keys(doc! { "email": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await?;
    info!("Ensured unique index on users.email");

    // ── 6. Build AppState ─────────────────────────────────────────────────────
    let state = AppState { db, config };

    // ── 7. CORS ───────────────────────────────────────────────────────────────
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    // ── 8. Assemble router ────────────────────────────────────────────────────
    let app = Router::new()
        .route("/health", get(health_handler))
        .nest_service("/uploads", ServeDir::new(&upload_dir))
        .merge(routes::build(state.clone()))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(max_upload_bytes))
        .with_state(state);

    // ── 9. Serve ──────────────────────────────────────────────────────────────
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
