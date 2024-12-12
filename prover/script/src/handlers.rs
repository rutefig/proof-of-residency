use crate::proof_service::ProofService;
use crate::types::ProofResponse;
use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use std::convert::Infallible;
use std::sync::Arc;
use warp::{
    filters::multipart::FormData,
    http::StatusCode,
    reject::Rejection,
    reply::Reply,
};

pub struct FileHandler {
    proof_service: Arc<ProofService>,
}

#[derive(Debug)]
pub enum HandlerError {
    InvalidFileType(String),
    FileReadError(String),
    ProofGenerationError(String),
}

impl warp::reject::Reject for HandlerError {}

impl FileHandler {
    pub fn new(proof_service: Arc<ProofService>) -> Self {
        Self { proof_service }
    }
    pub async fn handle_upload(&self, form: FormData) -> Result<impl Reply, Rejection> {
        let mut parts = form.into_stream();
        let mut file_content: Option<Vec<u8>> = None;
        let mut tx_hash: Option<String> = None;
    
        while let Some(Ok(p)) = parts.next().await {
            match p.name() {
                "file" => {
                    if let Some(file_type) = p.content_type() {
                        match file_type {
                            "application/pdf" => {
                                file_content = Some(self.read_file_content(p).await?);
                            }
                            _ => {
                                return Err(warp::reject::custom(HandlerError::InvalidFileType(
                                    file_type.to_string(),
                                )));
                            }
                        }
                    }
                }
                "tx_hash" => {
                    // Read tx_hash from form data
                    let bytes = self.read_file_content(p).await?;
                    tx_hash = Some(String::from_utf8(bytes)
                        .map_err(|e| warp::reject::custom(HandlerError::FileReadError(e.to_string())))?);
                }
                _ => {}
            }
        }
    
        if let (Some(content), Some(hash)) = (file_content, tx_hash) {
            let proof_response = self.generate_proof(content, hash).await?;
            Ok(warp::reply::json(&proof_response))
        } else {
            Err(warp::reject::custom(HandlerError::FileReadError(
                "Missing required fields".to_string(),
            )))
        }
    }

    async fn read_file_content(
        &self,
        part: warp::multipart::Part,
    ) -> Result<Vec<u8>, Rejection> {
        part.stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| {
                warp::reject::custom(HandlerError::FileReadError(e.to_string()))
            })
    }

    async fn generate_proof(&self, file_content: Vec<u8>, tx_hash: String) -> Result<ProofResponse, Rejection> {
        self.proof_service
            .generate_proof(file_content, tx_hash)
            .await
            .map_err(|e| {
                warp::reject::custom(HandlerError::ProofGenerationError(e.to_string()))
            })
    }
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else if let Some(e) = err.find::<HandlerError>() {
        match e {
            HandlerError::InvalidFileType(t) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid file type: {}", t),
            ),
            HandlerError::FileReadError(e) => (
                StatusCode::BAD_REQUEST,
                format!("Error reading file: {}", e),
            ),
            HandlerError::ProofGenerationError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error generating proof: {}", e),
            ),
        }
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}