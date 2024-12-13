use std::sync::Arc;
use std::time::Duration;
use tracing::info;

use crate::server_manager::ServerManager;

pub struct CleanupService {
    server_manager: Arc<ServerManager>,
    cleanup_interval: Duration,
    session_timeout: Duration,
}

impl CleanupService {
    pub fn new(
        server_manager: Arc<ServerManager>,
        cleanup_interval: Duration,
        session_timeout: Duration,
    ) -> Self {
        Self {
            server_manager,
            cleanup_interval,
            session_timeout,
        }
    }

    pub async fn start(self) {
        info!(
            "Starting cleanup service: interval={:?}, timeout={:?}",
            self.cleanup_interval,
            self.session_timeout
        );
        
        loop {
            tokio::time::sleep(self.cleanup_interval).await;
            self.server_manager
                .cleanup_inactive_sessions(self.session_timeout)
                .await;
        }
    }
}