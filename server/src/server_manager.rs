// src/server_manager.rs
use crate::error::ServerError;
use crate::models::{ProverInstance, SessionResponse};
use std::collections::HashMap;
use std::process::Child;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;
use tracing::info;

pub struct ServerManager {
    instances: Arc<Mutex<HashMap<String, ProverInstance>>>,
    config: Arc<crate::config::Config>,
}

impl ServerManager {
    pub fn new(config: Arc<crate::config::Config>) -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    pub async fn create_instance(&self) -> Result<SessionResponse, ServerError> {
        let session_id = Uuid::new_v4().to_string();
        let port = self.find_available_port()
            .map_err(|e| ServerError::PortError(e.to_string()))?;

        info!("Creating new prover instance on port {}", port);

        tokio::time::sleep(Duration::from_secs(1)).await;

        let process = self.spawn_prover_server(port)
            .map_err(|e| ServerError::ProverCreationError(e.to_string()))?;

        let instance = ProverInstance {
            process,
            port,
            last_active: Instant::now(),
        };

        let mut instances = self.instances.lock().await;
        instances.insert(session_id.clone(), instance);

        Ok(SessionResponse {
            session_id,
            prover_port: port,
        })
    }

    fn find_available_port(&self) -> Result<u16, std::io::Error> {
        let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
        Ok(socket.local_addr()?.port())
    }

    fn spawn_prover_server(&self, port: u16) -> Result<Child, std::io::Error> {
        std::process::Command::new("cargo")
            .current_dir("../prover/script")
            .env("SP1_PROVER", &self.config.sp1_prover)
            .env("SP1_PRIVATE_KEY", &self.config.sp1_private_key)
            .env("RUST_LOG", "info")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("--port")
            .arg(port.to_string())
            .spawn()
    }

    pub async fn cleanup_instance(&self, session_id: &str) -> Result<(), ServerError> {
        let mut instances = self.instances.lock().await;
        
        if let Some(mut instance) = instances.remove(session_id) {
            info!("Cleaning up instance for session {}", session_id);
            instance.process.kill()
                .map_err(|e| ServerError::Internal(e.to_string()))?;
            Ok(())
        } else {
            Err(ServerError::SessionNotFound(session_id.to_string()))
        }
    }

    pub async fn cleanup_inactive_sessions(&self, timeout: Duration) {
        let mut instances = self.instances.lock().await;
        let now = Instant::now();
        
        let inactive_sessions: Vec<String> = instances
            .iter()
            .filter(|(_, instance)| now.duration_since(instance.last_active) > timeout)
            .map(|(session_id, _)| session_id.clone())
            .collect();

        for session_id in inactive_sessions {
            if let Some(mut instance) = instances.remove(&session_id) {
                let _ = instance.process.kill(); // Ignore errors here
                info!("Cleaned up inactive session: {}", session_id);
            }
        }
    }

    pub async fn update_last_active(&self, session_id: &str) -> Result<(), ServerError> {
        let mut instances = self.instances.lock().await;
        
        if let Some(instance) = instances.get_mut(session_id) {
            instance.last_active = Instant::now();
            Ok(())
        } else {
            Err(ServerError::SessionNotFound(session_id.to_string()))
        }
    }
}