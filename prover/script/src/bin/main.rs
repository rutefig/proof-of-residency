use prover_script::{
    config::ProverConfig,
    handlers::{self, FileHandler},
    proof_service::{self, ProofService, ProverInstance},
};
use sp1_sdk::utils;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    utils::setup_logger();

    let args: Vec<String> = std::env::args().collect();

    // Get port from --port argument
    let port = args
        .iter()
        .position(|arg| arg == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let mut config = ProverConfig::default();
    config.port = port;

    let prover = Arc::new(ProverInstance::new());
    let verification_key = prover.verification_key();
    println!("Verification key: {}", verification_key); // Print it for now

    let proof_service = Arc::new(ProofService::new(Arc::clone(&prover)));
    let file_handler = Arc::new(FileHandler::new(Arc::clone(&proof_service)));

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(config.max_file_size))
        .and_then(move |form| {
            let handler = file_handler.clone();
            async move { handler.handle_upload(form).await }
        });

    let elf_route = {
        let prover = Arc::clone(&prover);
        warp::path("elf").and(warp::get()).map(move || {
            let elf_bytes = prover.get_elf();
            warp::reply::with_header(elf_bytes, "Content-Type", "application/octet-stream")
        })
    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let router = upload_route
        .or(elf_route)
        .recover(handlers::handle_rejection)
        .with(cors);

    println!("Server started at localhost:{}", config.port);
    warp::serve(router).run(([0, 0, 0, 0], config.port)).await;
}
