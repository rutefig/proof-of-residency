mod config;
mod error;
mod handlers;
mod models;
mod server_manager;
mod cleanup;

use cleanup::CleanupService;
use server_manager::ServerManager;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tracing::{info, Level};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config = Arc::new(config::Config::new().expect("Failed to load configuration"));
    let server_manager = Arc::new(server_manager::ServerManager::new(Arc::clone(&config)));

    // Start the cleanup service
    let cleanup_service = CleanupService::new(
        Arc::clone(&server_manager),
        Duration::from_secs(300),  // Clean up every 5 minutes
        Duration::from_secs(3600), // Session timeout after 1 hour
    );
    
    // Spawn the cleanup task
    tokio::spawn(cleanup_service.start());

    // Define routes
    let create_session = warp::path("session")
        .and(warp::post())
        .and(with_server_manager(Arc::clone(&server_manager)))
        .and_then(handlers::create_session_handler);

    let cleanup_session = warp::path!("session" / String)
        .and(warp::delete())
        .and(with_server_manager(Arc::clone(&server_manager)))
        .and_then(handlers::cleanup_session_handler);

    let heartbeat = warp::path!("session" / String / "heartbeat")
        .and(warp::post())
        .and(with_server_manager(Arc::clone(&server_manager)))
        .and_then(handlers::heartbeat_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "DELETE"])
        .allow_headers(vec!["content-type"]);

    let routes = create_session
        .or(cleanup_session)
        .or(heartbeat)
        .recover(handlers::handle_rejection)
        .with(cors);

    info!("Server starting on port {}", config.server_port);
    warp::serve(routes)
        .run(([127, 0, 0, 1], config.server_port))
        .await;
}

fn with_server_manager(
    server_manager: Arc<ServerManager>,
) -> impl Filter<Extract = (Arc<ServerManager>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&server_manager))
}
