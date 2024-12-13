use crate::error::ServerError;
use crate::proof_service::ProofService;
use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
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

impl FileHandler {
    pub fn new(proof_service: Arc<ProofService>) -> Self {
        Self { proof_service }
    }

    pub async fn handle_upload(&self, form: FormData) -> Result<impl Reply, Rejection> {
        let (file_content, tx_hash) = self.extract_form_data(form).await?;
        
        let proof_response = self.proof_service
            .generate_proof(file_content, tx_hash)
            .await
            .map_err(|e| warp::reject::custom(e))?;

        Ok(warp::reply::with_header(
            proof_response.proof,
            "Content-Type",
            "application/octet-stream"
        ))
    }

    async fn extract_form_data(&self, mut form: FormData) -> Result<(Vec<u8>, String), Rejection> {
        let mut file_content: Option<Vec<u8>> = None;
        let mut tx_hash: Option<String> = None;

        while let Some(Ok(part)) = form.next().await {
            match part.name() {
                "file" => {
                    if let Some(file_type) = part.content_type() {
                        match file_type {
                            "application/pdf" => {
                                file_content = Some(self.read_file_content(part).await?);
                            }
                            _ => {
                                return Err(warp::reject::custom(ServerError::InvalidFileType(
                                    file_type.to_string(),
                                )));
                            }
                        }
                    }
                }
                "tx_hash" => {
                    let bytes = self.read_file_content(part).await?;
                    tx_hash = Some(String::from_utf8(bytes)
                        .map_err(|e| warp::reject::custom(ServerError::FileReadError(e.to_string())))?);
                }
                _ => {}
            }
        }

        match (file_content, tx_hash) {
            (Some(content), Some(hash)) => Ok((content, hash)),
            _ => Err(warp::reject::custom(ServerError::FileReadError(
                "Missing required fields".to_string(),
            ))),
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
                warp::reject::custom(ServerError::FileReadError(e.to_string()))
            })
    }
}

pub async fn get_verification_key(
    proof_service: Arc<ProofService>,
) -> Result<impl Reply, Rejection> {
    let vk = proof_service.get_verification_key();
    Ok(warp::reply::json(&serde_json::json!({
        "verification_key": vk
    })))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else if let Some(e) = err.find::<ServerError>() {
        match e {
            ServerError::InvalidFileType(t) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid file type: {}", t),
            ),
            ServerError::FileReadError(e) => (
                StatusCode::BAD_REQUEST,
                format!("Error reading file: {}", e),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
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