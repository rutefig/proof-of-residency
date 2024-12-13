use crate::error::ServerError;
use crate::types::ProofResponse;
use sp1_sdk::{
    include_elf, network::proto::network::ProofMode, HashableKey, NetworkProverV1, ProverClient,
    SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin, SP1VerifyingKey,
};
use std::sync::Arc;

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
}

pub struct ProofService {
    prover: Arc<ProverInstance>,
}

impl ProofService {
    pub fn new(prover: Arc<ProverInstance>) -> Self {
        Self { prover }
    }

    pub fn get_verification_key(&self) -> String {
        self.prover.verification_key()
    }

    pub async fn generate_proof(
        &self,
        file_content: Vec<u8>,
        tx_hash: String,
    ) -> Result<ProofResponse, ServerError> {
        let mut stdin = SP1Stdin::new();

        stdin.write(&file_content);
        stdin.write(&tx_hash);

        let mut proof = match std::env::var("SP1_PROVER").as_deref() {
            Ok("network") => self.generate_network_proof(stdin).await?,
            _ => self.generate_local_proof(stdin)?,
        };

        let verification_result = proof.public_values.read::<bool>();
        proof
            .save("../../temp/proof-with-pis.bin")
            .map_err(|e| ServerError::Internal(e.to_string()))?;

        let proof_bytes = std::fs::read("../../temp/proof-with-pis.bin")
            .map_err(|e| ServerError::Internal(e.to_string()))?;

        Ok(ProofResponse {
            success: true,
            result: verification_result,
            proof: proof_bytes,
            tx_hash,
            vk: self.prover.verification_key(),
        })
    }

    fn generate_local_proof(
        &self,
        stdin: SP1Stdin,
    ) -> Result<SP1ProofWithPublicValues, ServerError> {
        self.prover
            .client
            .prove(&self.prover.pk, stdin)
            .compressed()
            .run()
            .map_err(|e| ServerError::Internal(e.to_string()))
    }

    async fn generate_network_proof(
        &self,
        stdin: SP1Stdin,
    ) -> Result<SP1ProofWithPublicValues, ServerError> {
        let private_key = std::env::var("SP1_PRIVATE_KEY")
            .map_err(|_| ServerError::Internal("SP1_PRIVATE_KEY not set".into()))?;

        let network = NetworkProverV1::new_from_key(&private_key);
        network
            .prove(REGEX_IO_ELF, stdin, ProofMode::Compressed, None)
            .await
            .map_err(|e| ServerError::Internal(e.to_string()))
    }
}
