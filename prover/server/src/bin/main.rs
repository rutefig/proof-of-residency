use prover_server::config::ServerConfig;
use prover_server::handlers::{self, FileHandler};
use prover_server::proof_service::{ProofService, ProverInstance};
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let config = ServerConfig::from_args();

    // Initialize prover
    let prover = Arc::new(ProverInstance::new());
    let proof_service = Arc::new(ProofService::new(Arc::clone(&prover)));
    let file_handler = Arc::new(FileHandler::new(Arc::clone(&proof_service)));

    // Setup routes
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(config.max_file_size))
        .and_then(move |form| {
            let handler = Arc::clone(&file_handler);
            async move { handler.handle_upload(form).await }
        });

    let verification_key_route = warp::path("verification-key")
        .and(warp::get())
        .and(with_proof_service(Arc::clone(&proof_service)))
        .and_then(handlers::get_verification_key);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let routes = upload_route
        .or(verification_key_route)
        .recover(handlers::handle_rejection)
        .with(cors);

    println!("Server started at localhost:{}", config.port);
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}

// Helper function to pass proof service to handlers
fn with_proof_service(
    proof_service: Arc<ProofService>,
) -> impl Filter<Extract = (Arc<ProofService>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&proof_service))
}
