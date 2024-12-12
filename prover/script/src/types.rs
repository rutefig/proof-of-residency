use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProofResponse {
    pub success: bool,
    pub result: bool,
    pub proof: Vec<u8>,
    pub tx_hash: String,
    pub vk: String,
}