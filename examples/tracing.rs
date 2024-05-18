use std::time::Duration;

use axum::{routing::get, Router};

use tokio::time::{sleep, Instant};
use tracing::level_filters::LevelFilter;
use tracing::{info, instrument, warn};

use tracing_subscriber::fmt::{self, format::FmtSpan};
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let level_sub = FmtSubscriber::builder()
    //     .with_max_level(LevelFilter::INFO)
    //     .finish();
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "track_req.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::WARN);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:8080";
    let app = Router::new().route("/", get(index_handler));

    info!("Srever listen at: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    anyhow::Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    sleep(Duration::from_millis(10)).await;
    let ret = long_runing().await;
    info!(http.status = 200, "Request done");
    ret
}

#[instrument]
async fn long_runing() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_millis(100)).await;
    let elapsed = start.elapsed().to_owned().as_millis();
    warn!(app.task_duration = elapsed, "long running task~");
    "Serve APP"
}
