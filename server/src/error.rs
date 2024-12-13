// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to create prover instance: {0}")]
    ProverCreationError(String),
    
    #[error("Failed to find available port: {0}")]
    PortError(String),
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl warp::reject::Reject for ServerError {}