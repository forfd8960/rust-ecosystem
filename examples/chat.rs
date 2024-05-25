use std::sync::Arc;
use std::{fmt, net::SocketAddr};

use dashmap::DashMap;
use futures::{stream::SplitStream, SinkExt, StreamExt}; // send
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::warn;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const MAX_MSG: usize = 128;

struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
}

#[derive(Debug, Clone)]
struct Message {
    sender: String,
    content: String,
}

#[derive(Debug)]
struct Peer {
    username: String,
    receiver: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080";
    info!("listen on: {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (conn, addr) = listener.accept().await?;
        info!("request from: {:?}", addr);

        // tokio::spawn(async move {})
    }
}

impl State {
    async fn broadcast(&self, addr: SocketAddr, msg: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }

            if let Err(e) = peer.value().send(msg.clone()).await {
                warn!("send msg err {}: {}", peer.key(), e);
            };
        }
    }

    async fn add_peer(
        &self,
        addr: SocketAddr,
        username: String,
        stream: Framed<TcpStream, LinesCodec>,
    ) -> Peer {
        let (tx, mut rx) = mpsc::channel(MAX_MSG);
        self.peers.insert(addr, tx);

        let (mut sender, receiver) = stream.split();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = sender.send(msg.clone().to_string()).await {
                    warn!("send to stream err: {}", e);
                }
            }
        });

        Peer { username, receiver }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            peers: DashMap::new(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.sender, self.content)
    }
}
