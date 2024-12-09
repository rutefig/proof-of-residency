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

        while let Some(Ok(p)) = parts.next().await {
            if p.name() == "file" {
                if let Some(file_type) = p.content_type() {
                    match file_type {
                        "application/pdf" => {
                            let file_content = self.read_file_content(p).await?;
                            let proof_response = self.generate_proof(file_content).await?;
                            return Ok(warp::reply::json(&proof_response));
                        }
                        _ => {
                            return Err(warp::reject::custom(HandlerError::InvalidFileType(
                                file_type.to_string(),
                            )));
                        }
                    }
                }
            }
        }

        Ok(warp::reply::json(&"success"))
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

    async fn generate_proof(&self, file_content: Vec<u8>) -> Result<ProofResponse, Rejection> {
        self.proof_service
            .generate_proof(file_content)
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