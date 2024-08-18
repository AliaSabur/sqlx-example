use axum::{
    routing::{get, post}, Router
};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod config;
mod db;
pub mod error;
mod handlers;
mod signal;

use crate::config::Config;
use anyhow::Result;
use axum_server::Handle;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[tokio::main]
async fn main() -> Result<()> {
    // Load the configuration from the file
    let config = Config::from_file("config.yaml")?;

    // Initialize the tracing subscriber
    let filter = EnvFilter::from_default_env().add_directive(
        if config.debug {
            Level::DEBUG
        } else {
            Level::INFO
        }
        .into(),
    );
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder().with_env_filter(filter).finish(),
    )?;

    tracing::info!("Listening on http(s)://{}", config.server.bind);

    // Initialize the database connection pool
    let pool = db::establish_connection(&config.database.url).await?;

    // Create a new router
    let router = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::WARN)),
        )
        .layer(
            CorsLayer::new()
                .allow_credentials(true)
                .allow_headers(AllowHeaders::mirror_request())
                .allow_methods(AllowMethods::mirror_request())
                .allow_origin(AllowOrigin::mirror_request()),
        )
        .route("/upload", post(handlers::upload_handler))
        .route("/get", get(handlers::get_handler))
        .route("/*path", get(handlers::get_static_resource))
        .with_state(pool);

    // Create a new Handle for graceful shutdown.
    let handle = Handle::new();

    // Spawn a task to shutdown server.
    tokio::spawn(signal::graceful_shutdown(handle.clone()));

    axum_server::Server::bind(config.server.bind)
        .handle(handle)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
