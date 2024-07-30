use axum::serve;
use routes::app::app;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use utils::log::trace_layer_on_request;

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = app().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_request(trace_layer_on_request),
    );
    let address = "127.0.0.1:18080";
    let listener = TcpListener::bind(address).await.unwrap();

    info!("Listening on http://{}", address);

    serve(listener, app.into_make_service()).await.unwrap();
}
