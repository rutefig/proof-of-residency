use warp::{reject::Rejection, Filter};

mod handlers;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| "Welcome to my warp server!");
    let get_hello_route = warp::path("hello")
        .and(warp::get())
        .and_then(handlers::get_hello);

    let routes = root
        .or(get_hello_route)
        .with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}
