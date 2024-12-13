use prover_server::handlers::{self, FileHandler};
use prover_server::proof_service::{ProofService, ProverInstance};
use prover_server::config::ServerConfig;
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

    let elf_route = {
        let prover = Arc::clone(&prover);
        warp::path("elf")
            .and(warp::get())
            .map(move || {
                let elf_bytes = prover.get_elf();
                warp::reply::with_header(
                    elf_bytes,
                    "Content-Type",
                    "application/octet-stream",
                )
            })
    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let routes = upload_route
        .or(elf_route)
        .recover(handlers::handle_rejection)
        .with(cors);

    println!("Server started at localhost:{}", config.port);
    warp::serve(routes)
        .run(([0, 0, 0, 0], config.port))
        .await;
}