use warp::Filter;
use std::sync::Arc;
use serde_json::json;

mod server_manager;
use server_manager::ServerManager;

// Custom error type that implements Reject
#[derive(Debug)]
struct ServerError(String);
impl warp::reject::Reject for ServerError {}

#[tokio::main]
async fn main() {
    let server_manager = Arc::new(ServerManager::new());
    
    // Create new prover session
    let create_session = {
        let server_manager = Arc::clone(&server_manager);
        warp::path("session")
            .and(warp::post())
            .and_then(move || {
                let server_manager = Arc::clone(&server_manager);
                async move {
                    match server_manager.create_instance().await {
                        Ok((session_id, port)) => Ok(warp::reply::json(&json!({
                            "session_id": session_id,
                            "prover_port": port
                        }))),
                        Err(e) => Err(warp::reject::custom(ServerError(e.to_string())))
                    }
                }
            })
    };

    // Cleanup session
    let cleanup_session = {
        let server_manager = Arc::clone(&server_manager);
        warp::path!("session" / String)
            .and(warp::delete())
            .and_then(move |session_id: String| {
                let server_manager = Arc::clone(&server_manager);
                async move {
                    match server_manager.cleanup_instance(&session_id).await {
                        Ok(()) => Ok(warp::reply::json(&"Session cleaned up")),
                        Err(e) => Err(warp::reject::custom(ServerError(e.to_string())))
                    }
                }
            })
    };

    // Handle rejection (errors)
    async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
        let code;
        let message;

        if err.is_not_found() {
            code = warp::http::StatusCode::NOT_FOUND;
            message = "Not Found";
        } else if let Some(server_error) = err.find::<ServerError>() {
            code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
            message = &server_error.0;
        } else {
            code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
        }

        Ok(warp::reply::with_status(
            warp::reply::json(&json!({"error": message})),
            code,
        ))
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "DELETE"])
        .allow_headers(vec!["content-type"]);

    let routes = create_session
        .or(cleanup_session)
        .recover(handle_rejection)
        .with(cors);

    println!("Server manager started on port 8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}