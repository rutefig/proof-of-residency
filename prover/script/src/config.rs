use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProverConfig {
    pub max_file_size: u64,
    pub port: u16,
    pub hyle_base_path: String,
    pub proof_output_path: String,
}

impl Default for ProverConfig {
    fn default() -> Self {
        Self {
            max_file_size: 5_000_000,
            port: 8080,
            hyle_base_path: std::env::var("HYLE_BASE_PATH").expect("HYLE_BASE_PATH not set"),
            proof_output_path: "proof-with-pis.bin".to_string(),
        }
    }
}