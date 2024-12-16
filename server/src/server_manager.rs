// src/server_manager.rs
use crate::error::ServerError;
use crate::models::{ProverInstance, SessionResponse};
use std::collections::HashMap;
use std::process::Child;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use tracing::info;
use uuid::Uuid;

pub struct ServerManager {
    instances: Arc<Mutex<HashMap<String, ProverInstance>>>,
    warm_pool: Arc<Mutex<Vec<(Child, u16)>>>, // (process, port)
    config: Arc<crate::config::Config>,
    pool_semaphore: Arc<Semaphore>,
}

impl ServerManager {
    pub fn new(config: Arc<crate::config::Config>) -> Self {
        let manager = Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            warm_pool: Arc::new(Mutex::new(Vec::new())),
            config: Arc::clone(&config),
            pool_semaphore: Arc::new(Semaphore::new(3)), // Maintain 3 warm servers
        };

        // Start background task to maintain warm pool
        manager.start_warm_pool_maintenance();
        manager
    }

    fn start_warm_pool_maintenance(&self) {
        let warm_pool = Arc::clone(&self.warm_pool);
        let semaphore = Arc::clone(&self.pool_semaphore);
        let config = Arc::clone(&self.config);

        tokio::spawn(async move {
            loop {
                let permit = semaphore.acquire().await.unwrap();
                let port = Self::find_available_port().unwrap();
                let process = Self::spawn_prover_server(port, &config).unwrap();

                let mut pool = warm_pool.lock().await;
                pool.push((process, port));

                // Don't drop the permit - it represents a warm server
                std::mem::forget(permit);

                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });
    }

    pub async fn create_instance(&self) -> Result<SessionResponse, ServerError> {
        let session_id = Uuid::new_v4().to_string();

        // Try to get a warm server first
        let instance = {
            let mut warm_pool = self.warm_pool.lock().await;
            if let Some((process, port)) = warm_pool.pop() {
                ProverInstance {
                    process,
                    port,
                    last_active: Instant::now(),
                }
            } else {
                // Fall back to creating a new server if warm pool is empty
                let port = Self::find_available_port()
                    .map_err(|e| ServerError::PortError(e.to_string()))?;
                let process = Self::spawn_prover_server(port, &self.config)
                    .map_err(|e| ServerError::ProverCreationError(e.to_string()))?;
                ProverInstance {
                    process,
                    port,
                    last_active: Instant::now(),
                }
            }
        };

        let mut instances = self.instances.lock().await;
        instances.insert(session_id.clone(), instance);

        Ok(SessionResponse {
            session_id: session_id.clone(),
            prover_port: instances[&session_id].port,
        })
    }

    fn find_available_port() -> Result<u16, std::io::Error> {
        let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
        Ok(socket.local_addr()?.port())
    }

    fn spawn_prover_server(
        port: u16,
        config: &crate::config::Config,
    ) -> Result<Child, std::io::Error> {
        std::process::Command::new("../prover/target/release/prover")
            .env("SP1_PROVER", &config.sp1_prover)
            .env("SP1_PRIVATE_KEY", &config.sp1_private_key)
            .env("RUST_LOG", "info")
            .arg("--port")
            .arg(port.to_string())
            .spawn()
    }

    pub async fn cleanup_instance(&self, session_id: &str) -> Result<(), ServerError> {
        let mut instances = self.instances.lock().await;

        if let Some(mut instance) = instances.remove(session_id) {
            info!("Cleaning up instance for session {}", session_id);
            instance
                .process
                .kill()
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
