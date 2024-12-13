// src/models.rs
use serde::{Deserialize, Serialize};
use std::process::Child;
use std::time::Instant;

#[derive(Debug)]
pub struct ProverInstance {
    pub process: Child,
    pub port: u16,
    pub last_active: Instant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub session_id: String,
    pub prover_port: u16,
}