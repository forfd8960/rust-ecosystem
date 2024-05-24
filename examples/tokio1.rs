#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hello().await;
    anyhow::Ok(())
}

async fn hello() {
    println!("Running in Runtime");
}
