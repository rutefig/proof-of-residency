use crate::types::ProofResponse;
use crate::hyle::{Hyle, HyleNetwork};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1Stdin
};
use std::path::PathBuf;

/// The ELF we want to execute inside the zkVM.
const REGEX_IO_ELF: &[u8] = include_elf!("prover-program");

pub struct ProofService {
    hyle: Hyle,
}

impl ProofService {
    pub fn new(base_path: &PathBuf) -> Self {
        Self {
            hyle: Hyle::new(HyleNetwork::Devnet, base_path),
        }
    }

    pub async fn generate_proof(&self, file_content: Vec<u8>) -> Result<ProofResponse, Box<dyn std::error::Error>> {
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

        // Local prover implementation
        let client = ProverClient::new();
        let (pk, vk) = client.setup(REGEX_IO_ELF);
        println!("vk: {:?}", vk.bytes32());
        
        let mut proof = client.prove(&pk, stdin).run()?;
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
}