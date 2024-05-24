use std::time::Duration;
use tokio::time::sleep;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(my_future());
}

async fn my_future() {
    let d = Duration::from_millis(100);
    sleep(d).await;
    println!("running a future after: {:?}", d);
}
