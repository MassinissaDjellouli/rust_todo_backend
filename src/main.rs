use axum::Router;
use std::env;
mod bin;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let routes = bin::routes::routes_impl::Routes::new();
    let app = Router::new()
        .route("/user", axum::routing::post(routes.get_route("/user")));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().expect("There should be an address"));
    axum::serve(listener, app).await.expect("The server should run");

}
