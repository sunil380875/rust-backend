use mongodb::Database;
use crate::config::Config;

/// Shared application state injected into every handler via Axum's `State` extractor.
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Config,
}
