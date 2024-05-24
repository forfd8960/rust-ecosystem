use std::{
    thread::{self, sleep},
    time::Duration,
};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handle = worker(rx);

    tokio::spawn(async move {
        loop {
            tx.send("Main Message".to_string()).await?;
        }

        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(()) // set the return error type
    });

    handle.join().unwrap();
    anyhow::Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while let Some(data) = rx.blocking_recv() {
            let ret = long_run_future(data);
            println!("{:?}", ret);
        }
    })
}

fn long_run_future(s: String) -> String {
    let d = Duration::from_millis(5);
    sleep(d);
    println!("running a future after: {:?}, {}", d, s);
    format!("running a future after: {:?}, {}", d, s)
}
