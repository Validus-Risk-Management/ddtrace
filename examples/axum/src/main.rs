use std::net::SocketAddr;
use std::time::Duration;

use axum::{routing::get, Router};
use ddtrace::axum::OtelAxumLayer;
use ddtrace::error::Error;
use ddtrace::formatter::DatadogFormatter;
use ddtrace::set_global_propagator;
use ddtrace::tracer::build_layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service_name = std::env::var("DD_SERVICE").unwrap_or("my-service".to_string());
    let (tracing_layer, _guard) = build_layer(service_name)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .event_format(DatadogFormatter),
        )
        .with(tracing_layer)
        .init();
    set_global_propagator();

    let app = Router::new()
        .route("/", get(root))
        .layer(OtelAxumLayer::default())
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3025));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::info!("listening on {}", addr);

    Ok(())
}

async fn root() -> &'static str {
    do_something().await;
    "Hello, World!"
}

#[tracing::instrument]
async fn do_something() {
    tokio::time::sleep(Duration::from_millis(120)).await;
    do_something_else().await;
    tracing::info!("in the middle of doing something");
    tokio::time::sleep(Duration::from_millis(10)).await;
    do_something_else().await;
    tokio::time::sleep(Duration::from_millis(20)).await;
}

#[tracing::instrument]
async fn do_something_else() {
    tokio::time::sleep(Duration::from_millis(40)).await;
}

async fn health() -> &'static str {
    "healthy"
}
