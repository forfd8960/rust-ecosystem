use std::sync::Arc;
use std::sync::Mutex;

use axum::routing::get;
use axum::routing::patch;
use axum::Router;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::instrument;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct News {
    title: String,
    author: String,
    content: String,
    pub_date: i64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    let news = News {
        title: "".to_string(),
        author: "".to_string(),
        content: "".to_string(),
        pub_date: Utc::now().timestamp(),
    };

    let atomic_news = Arc::new(Mutex::new(news));

    let addr = "0.0.0.0:6379";
    let listner = TcpListener::bind(addr).await?;
    info!("listen on: {}", addr);

    let app = Router::new()
        .route("/", get(get_news))
        .route("/", patch(update_news))
        .with_state(atomic_news);
    axum::serve(listner, app.into_make_service()).await?;

    anyhow::Ok(())
}

#[instrument]
async fn get_news() {}

#[instrument]
async fn update_news() {}
