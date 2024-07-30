use axum::serve;
use routes::app::app;
use tokio::net::TcpListener;
use tracing::info;

mod routes;

#[tokio::main]
async fn main() {
    let app = app();
    let address = "127.0.0.1:18080";
    let listener = TcpListener::bind(address).await.unwrap();

    info!("Listening on http://{}", address);

    serve(listener, app.into_make_service()).await.unwrap();
}
