use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};

use chrono::{DateTime, Utc};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use tokio::net::TcpListener;
use tracing::instrument;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct News {
    title: String,
    author: String,
    content: String,
    pub_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
struct NewsUpdate {
    content: Option<String>,
    pub_date: Option<DateTime<Utc>>,
}

impl Serialize for News {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut data = serializer.serialize_struct("News", 4)?;
        data.serialize_field("title", &self.title)?;
        data.serialize_field("author", &self.author)?;
        data.serialize_field("content", &self.content)?;
        data.serialize_field("pub_date", &self.pub_date)?;
        data.end()
    }
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
        title: "why learn rust".to_string(),
        author: "someone".to_string(),
        content: "rust is awesome".to_string(),
        pub_date: Utc::now(),
    };

    let addr = "0.0.0.0:8080";
    let listner = TcpListener::bind(addr).await?;
    info!("listen on: {}", addr);

    let atomic_news = Arc::new(Mutex::new(news));
    let app = Router::new()
        .route("/", get(get_news))
        .route("/", patch(update_news))
        .with_state(atomic_news);
    axum::serve(listner, app.into_make_service()).await?;

    anyhow::Ok(())
}

#[instrument]
async fn get_news(State(news): State<Arc<Mutex<News>>>) -> Json<News> {
    (*news.lock().unwrap()).clone().into()
}

#[instrument]
async fn update_news(
    State(news): State<Arc<Mutex<News>>>,
    Json(news_update): Json<NewsUpdate>,
) -> Json<News> {
    let mut news = news.lock().unwrap();
    if let Some(content) = news_update.content {
        news.content = content;
    }
    if let Some(pub_date) = news_update.pub_date {
        news.pub_date = pub_date;
    }

    (*news).clone().into()
}
