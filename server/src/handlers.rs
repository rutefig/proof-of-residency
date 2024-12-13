// src/handlers.rs
use crate::error::ServerError;
use crate::server_manager::ServerManager;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Reply, Rejection};
use tracing::error;

pub async fn create_session_handler(
    server_manager: Arc<ServerManager>,
) -> Result<impl Reply, Rejection> {
    match server_manager.create_instance().await {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(e) => {
            error!("Failed to create session: {:?}", e);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn cleanup_session_handler(
    session_id: String,
    server_manager: Arc<ServerManager>,
) -> Result<impl Reply, Rejection> {
    match server_manager.cleanup_instance(&session_id).await {
        Ok(()) => Ok(warp::reply::json(&"Session cleaned up successfully")),
        Err(e) => {
            error!("Failed to cleanup session {}: {:?}", session_id, e);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn heartbeat_handler(
    session_id: String,
    server_manager: Arc<ServerManager>,
) -> Result<impl Reply, Rejection> {
    match server_manager.update_last_active(&session_id).await {
        Ok(()) => Ok(warp::reply::json(&"Session updated")),
        Err(e) => {
            error!("Failed to update session {}: {:?}", session_id, e);
            Err(warp::reject::custom(e))
        }
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (warp::http::StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<ServerError>() {
        match e {
            ServerError::SessionNotFound(_) => 
                (warp::http::StatusCode::NOT_FOUND, e.to_string()),
            ServerError::ProverCreationError(_) | ServerError::PortError(_) =>
                (warp::http::StatusCode::SERVICE_UNAVAILABLE, e.to_string()),
            _ => (warp::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    } else {
        (warp::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "error": message
        })),
        code,
    ))
}