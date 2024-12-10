use crate::hyle::{Hyle, HyleNetwork};
use crate::types::ProofResponse;
use sp1_sdk::network::proto::network::ProofMode;
use sp1_sdk::{
    include_elf, HashableKey, NetworkProverV1, ProverClient, SP1ProofWithPublicValues, SP1Stdin,
};
use std::path::PathBuf;

/// The ELF we want to execute inside the zkVM.
const REGEX_IO_ELF: &[u8] = include_elf!("prover-program");

pub struct ProofService {
    hyle: Hyle,
}

enum Prover {
    Local,
    Network,
}

impl Prover {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "network" => Prover::Network,
            _ => Prover::Local, // Default to Local for any other value
        }
    }
}

impl ProofService {
    pub fn new(base_path: &PathBuf) -> Self {
        Self {
            hyle: Hyle::new(HyleNetwork::Devnet, base_path),
        }
    }

    pub async fn generate_proof(
        &self,
        file_content: Vec<u8>,
    ) -> Result<ProofResponse, Box<dyn std::error::Error>> {
        let mut stdin = SP1Stdin::new();

        let null_state = 0u32.to_be_bytes().to_vec();
        let hyle_input = self.hyle.publish_payload(
            "default",
            "residency",
            "Portugal",
            &null_state,
            &file_content,
        )?;

        stdin.write(&hyle_input);

        let prover = std::env::var("SP1_PROVER")
            .map(|s| Prover::from_str(&s))
            .unwrap_or(Prover::Local);

        let mut proof = match prover {
            Prover::Local => self.generate_proof_local(stdin),
            Prover::Network => self.generate_proof_network(stdin).await,
        };

        let verification_result = proof.public_values.read::<bool>();

        // Save proof to file
        proof.save("proof-with-pis.bin")?;
        let proof_bytes = std::fs::read("proof-with-pis.bin")?;

        Ok(ProofResponse {
            success: true,
            result: verification_result,
            proof: proof_bytes,
        })
    }

    fn generate_proof_local(&self, stdin: SP1Stdin) -> SP1ProofWithPublicValues {
        let client = ProverClient::new();
        let (pk, vk) = client.setup(REGEX_IO_ELF);
        println!("vk: {:?}", vk.bytes32());

        let proof = client.prove(&pk, stdin).run().unwrap();

        proof
    }

    async fn generate_proof_network(&self, stdin: SP1Stdin) -> SP1ProofWithPublicValues {
        let network = NetworkProverV1::new_from_key(
            &std::env::var("SP1_NETWORK_KEY").expect("SP1_NETWORK_KEY not set"),
        );
        let proof = network
            .prove(REGEX_IO_ELF, stdin, ProofMode::Core, None)
            .await
            .unwrap();

        proof
    }
}
