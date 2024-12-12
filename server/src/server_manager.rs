use std::collections::HashMap;
use std::process::Child;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use std::time::{Duration, Instant};

pub struct ProverInstance {
    pub process: Child,
    pub port: u16,
    pub last_active: Instant,
}

pub struct ServerManager {
    instances: Arc<Mutex<HashMap<String, ProverInstance>>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_instance(&self) -> Result<(String, u16), Box<dyn std::error::Error>> {
        let session_id = Uuid::new_v4().to_string();
        let port = self.find_available_port()?;

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        let process = self.spawn_prover_server(port)?;
        
        let instance = ProverInstance {
            process,
            port,
            last_active: Instant::now(),
        };

        let mut instances = self.instances.lock().await;
        instances.insert(session_id.clone(), instance);

        Ok((session_id, port))
    }

    fn find_available_port(&self) -> Result<u16, Box<dyn std::error::Error>> {
        let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = socket.local_addr()?.port();
        Ok(port)
    }

    fn spawn_prover_server(&self, port: u16) -> Result<Child, Box<dyn std::error::Error>> {
        let process = std::process::Command::new("cargo")
            .current_dir("../prover/script")
            .env("RUST_LOG", "info")
            .arg("run")
            .arg("--release")
            .arg("--")
            .arg("--execute")
            .arg("--port")
            .arg(port.to_string())
            .spawn()?;
        Ok(process)
    }

    pub async fn cleanup_instance(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut instances = self.instances.lock().await;
        if let Some(mut instance) = instances.remove(session_id) {
            instance.process.kill()?;
        }
        Ok(())
    }

    pub async fn update_last_active(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut instances = self.instances.lock().await;
        if let Some(instance) = instances.get_mut(session_id) {
            instance.last_active = Instant::now();
        }
        Ok(())
    }
}