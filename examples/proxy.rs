use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = get_config();
    println!("listen: {}", conf.listen_addr);
    println!("upstream: {}", conf.upstream_addr);

    let listen_addr = conf.listen_addr.clone();
    let con_conf = Arc::new(conf);

    let listener = TcpListener::bind(&listen_addr).await?;
    loop {
        let (conn, addr) = listener.accept().await?;
        println!("request from: {:?}", addr);

        let cloned_conf = Arc::clone(&con_conf);

        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_conf.upstream_addr).await?;

            proxy(conn, upstream).await?;

            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(()) // set the return error type
        });
    }
}

fn get_config() -> Config {
    Config {
        listen_addr: "0.0.0.0:8080".to_string(),
        upstream_addr: "0.0.0.0:8081".to_string(),
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> anyhow::Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();

    let read_from_client = tokio::io::copy(&mut client_reader, &mut upstream_writer);
    let write_to_client = tokio::io::copy(&mut upstream_reader, &mut client_writer);

    tokio::try_join!(read_from_client, write_to_client)?;
    anyhow::Ok(())
}
