use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProofResponse {
    pub success: bool,
    pub result: bool,
    pub proof: Vec<u8>,
}