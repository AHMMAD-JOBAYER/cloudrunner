mod auth;
mod config;
mod db;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    config: Config,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "university_nixos_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Create database pool
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("auto sqlx migration failed!");

    tracing::info!("Database connection established");

    // Create app state
    let state = AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Configure CORS for frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        // Auth routes
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/logout", post(handlers::logout))
        .route(
            "/api/auth/reset-password",
            post(handlers::request_password_reset),
        )
        .route(
            "/api/auth/reset-password/confirm",
            post(handlers::confirm_password_reset),
        )
        .route("/api/auth/me", get(handlers::get_current_teacher))
        // NixOS config routes
        .route("/api/configs", post(handlers::upload_config))
        .route("/api/configs", get(handlers::list_configs))
        .route("/api/configs/{id}", get(handlers::get_config))
        .route("/api/configs/{id}", put(handlers::update_config))
        .route("/api/configs{id}", delete(handlers::delete_config))
        .layer(cors)
        .with_state(state);

    // Start server
    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server starting on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn health_check() -> &'static str {
    "OK"
}
