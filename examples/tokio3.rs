use std::{thread, time::Duration};

use tokio::runtime::Builder;
use tokio::time::sleep;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello, Thread");
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Future1");
        });

        rt.spawn(async {
            println!("Future2");
        });
        rt.block_on(long_run_future())
    });

    handle.join().unwrap();
}

async fn long_run_future() {
    let d = Duration::from_millis(100);
    sleep(d).await;
    println!("running a future after: {:?}", d);
}
