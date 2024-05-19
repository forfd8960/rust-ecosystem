use std::time::Duration;

use axum::{routing::get, Router};

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self, RandomIdGenerator, Tracer},
    Resource,
};

use tokio::{
    join,
    time::{sleep, Instant},
};
use tracing::level_filters::LevelFilter;
use tracing::{info, instrument, warn};

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

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

    // let tracer = init_tracer()?;
    // let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        // .with(opentelemetry)
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

    let t1 = task1();
    let t2 = task2();
    let t3 = task3();
    join!(t1, t2, t3);

    let elapsed = start.elapsed().to_owned().as_millis();
    warn!(app.task_duration = elapsed, "long running task~");
    "Serve APP"
}

#[instrument]
async fn task1() {
    sleep(Duration::from_millis(10)).await;
}

#[instrument]
async fn task2() {
    sleep(Duration::from_millis(20)).await;
}

#[instrument]
async fn task3() {
    sleep(Duration::from_millis(30)).await;
}

// #[allow_deadcode]
fn init_tracer() -> anyhow::Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(32)
                .with_max_attributes_per_span(64)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(runtime::Tokio)?;

    anyhow::Ok(tracer)
}
