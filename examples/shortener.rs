use axum::{
    extract::{Path, State},
    http::{header::LOCATION, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const LISTEN_ADDR: &str = "127.0.0.1:8088";

#[derive(Debug, Serialize, Deserialize)]
struct ShortRequest {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortResponse {
    url: String,
}

#[derive(Debug, FromRow)]
struct UrlRecord {
    #[sqlx(default)]
    id: String,
    #[sqlx(default)]
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    info!("listen on: {}", LISTEN_ADDR);
    let listener = TcpListener::bind(LISTEN_ADDR).await?;

    let url = "postgres://postgres:postgres@localhost:5432/shortener";
    let state = AppState::try_new(url).await?;
    let app = Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await?;
    anyhow::Ok(())
}

async fn shorten(
    State(state): State<AppState>,
    Json(data): Json<ShortRequest>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let id = state.shorten(&data.url).await.map_err(|e| {
        warn!("Failed to shorten url: {e}");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;

    let body = Json(ShortResponse {
        url: format!("http://{}/{}", LISTEN_ADDR, id),
    });

    Ok((StatusCode::CREATED, body))
}

async fn redirect(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> anyhow::Result<impl IntoResponse, StatusCode> {
    let url = state.get_url(id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}

impl AppState {
    async fn try_new(url: &str) -> anyhow::Result<Self> {
        let pool = PgPool::connect(url).await?;
        sqlx::query(r#""#).execute(&pool).await?;

        anyhow::Ok(Self { db: pool })
    }

    async fn shorten(&self, url: &str) -> anyhow::Result<String> {
        let id = nanoid!(6);
        let ret: UrlRecord = sqlx::query_as(
            "INSERT INTO urls (id, url) VALUES ($1, $2) ON CONFLICT(url) DO UPDATE SET url=EXCLUDED.url RETURNING id".as_ref(),
        ).bind(&id).bind(url).fetch_one(&self.db).await?;

        Ok(ret.id)
    }

    async fn get_url(&self, id: String) -> anyhow::Result<String> {
        let ret: UrlRecord = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(ret.url)
    }
}
