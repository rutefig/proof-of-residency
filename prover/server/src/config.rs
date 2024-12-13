use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub max_file_size: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            max_file_size: 5_000_000, // 5MB
        }
    }
}

impl ServerConfig {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let port = args
            .iter()
            .position(|arg| arg == "--port")
            .and_then(|i| args.get(i + 1))
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        Self {
            port,
            ..Default::default()
        }
    }
}