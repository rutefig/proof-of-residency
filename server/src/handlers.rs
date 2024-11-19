use crate::Result;
use warp::{http::StatusCode, reply, Reply};


pub async fn get_hello() -> Result<impl Reply> {
    let response = String::from("Hello World");
    Ok(reply::with_status(reply::json(&response), StatusCode::OK))
}