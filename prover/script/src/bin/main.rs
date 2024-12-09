use prover_script::{
    config::ProverConfig,
    handlers::{self, FileHandler},
    proof_service::ProofService,
};
use sp1_sdk::utils;
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    utils::setup_logger();

    let config = ProverConfig::default();
    let home_dir = std::env::home_dir().unwrap();
    let mut base_path = home_dir.clone();
    base_path.push(&config.hyle_base_path);

    let proof_service = Arc::new(ProofService::new(&base_path));
    let file_handler = Arc::new(FileHandler::new(proof_service));

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(config.max_file_size))
        .and_then(move |form| {
            let handler = file_handler.clone();
            async move { handler.handle_upload(form).await }
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let router = upload_route.recover(handlers::handle_rejection).with(cors);

    println!("Server started at localhost:{}", config.port);
    warp::serve(router).run(([0, 0, 0, 0], config.port)).await;
}
