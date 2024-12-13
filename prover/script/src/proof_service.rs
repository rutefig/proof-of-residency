use std::sync::Arc;

use crate::types::ProofResponse;
use base64::prelude::*;
use hyle_contract::HyleInput;
use sp1_sdk::network::proto::network::ProofMode;
use sp1_sdk::{
    include_elf, HashableKey, NetworkProverV1, ProverClient, SP1ProofWithPublicValues,
    SP1ProvingKey, SP1Stdin, SP1VerifyingKey,
};

/// The ELF we want to execute inside the zkVM.
const REGEX_IO_ELF: &[u8] = include_elf!("prover-program");

pub struct ProverInstance {
    pk: SP1ProvingKey,
    vk: SP1VerifyingKey,
    client: ProverClient,
}

impl ProverInstance {
    pub fn new() -> Self {
        let client = ProverClient::new();
        let (pk, vk) = client.setup(REGEX_IO_ELF);
        println!("VK: {}", vk.bytes32());
        Self { pk, vk, client }
    }

    pub fn verification_key(&self) -> String {
        self.vk.bytes32()
    }

    pub fn get_elf(&self) -> Vec<u8> {
        REGEX_IO_ELF.to_vec()
    }
}

pub struct ProofService {
    prover: Arc<ProverInstance>,
}

enum Prover {
    Local,
    Network,
}

impl Prover {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "network" => Prover::Network,
            _ => Prover::Local,
        }
    }
}

impl ProofService {
    pub fn new(prover: Arc<ProverInstance>) -> Self {
        Self { prover }
    }

    pub async fn generate_proof(
        &self,
        file_content: Vec<u8>,
        tx_hash: String,
    ) -> Result<ProofResponse, Box<dyn std::error::Error>> {
        let mut stdin = SP1Stdin::new();

        let null_state = 0u32.to_be_bytes().to_vec();
        let tx_hash_bytes =
            hex::decode(&tx_hash).map_err(|e| format!("Failed to decode tx_hash hex: {}", e))?;

        let hyle_input = HyleInput {
            identity: "".to_string(),
            initial_state: null_state,
            program_inputs: &file_content,
            tx_hash: tx_hash_bytes,
        };

        stdin.write(&hyle_input);

        let prover = std::env::var("SP1_PROVER")
            .map(|s| Prover::from_str(&s))
            .unwrap_or(Prover::Local);

        let mut proof = match prover {
            Prover::Local => self.generate_proof_local(stdin),
            Prover::Network => self.generate_proof_network(stdin).await,
        };

        let verification_result = proof.public_values.read::<bool>();

        // proof.save("../../temp/proof-with-pis.bin")?;
        proof.save("../../temp/proof-with-pis.bin")?;

        let proof_bytes = std::fs::read("../../temp/proof-with-pis.bin")?;

        Ok(ProofResponse {
            success: true,
            result: verification_result,
            proof: proof_bytes,
            tx_hash: hex::encode(hyle_input.tx_hash),
            vk: self.prover.verification_key(),
        })
    }

    fn generate_proof_local(&self, stdin: SP1Stdin) -> SP1ProofWithPublicValues {
        self.prover
            .client
            .prove(&self.prover.pk, stdin)
            .compressed()
            .run()
            .unwrap()
    }

    async fn generate_proof_network(&self, stdin: SP1Stdin) -> SP1ProofWithPublicValues {
        let network = NetworkProverV1::new_from_key(
            &std::env::var("SP1_PRIVATE_KEY").expect("SP1_PRIVATE_KEY not set"),
        );
        network
            .prove(REGEX_IO_ELF, stdin, ProofMode::Core, None)
            .await
            .unwrap()
    }

    fn mock_proof_response(hyle_input: HyleInput<Vec<u8>>) -> ProofResponse {
        let proof_bytes = std::fs::read("proof-with-pis.bin").expect("Failed to read proof file");
        ProofResponse {
            success: true,
            result: true,
            proof: proof_bytes,
            tx_hash: hex::encode(hyle_input.tx_hash),
            vk: "mock_vk".to_string(),
        }
    }
}
