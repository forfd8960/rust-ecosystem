use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Debug, Serialize, Deserialize)]
struct ShortRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortResponse {
    id: String,
}

#[derive(Debug)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080";
    info!("listen on: {}", addr);

    let listener = TcpListener::bind(addr).await?;

    let app = Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect));

    axum::serve(listener, app.into_make_service()).await?;
    anyhow::Ok(())
}

async fn shorten(Json(data): Json<ShortRequest>) -> Result<Json<ShortResponse>, StatusCode> {
    todo!()
}

async fn redirect(Path(id): Path<String>) {}
