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

#[derive(Debug)]
struct State {
    peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
}

#[derive(Debug, Clone)]
enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

#[derive(Debug)]
struct Peer {
    username: String,
    receiver: SplitStream<Framed<TcpStream, LinesCodec>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    console_subscriber::init();

    // let layer = Layer::new().with_filter(LevelFilter::INFO);
    // tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080";
    info!("listen on: {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let state = Arc::new(State::default());

    loop {
        let (client_conn, addr) = listener.accept().await?;
        info!("client conn from: {:?}", addr);

        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(state_clone, addr, client_conn).await {
                warn!("handle client error: {}", e);
            }
        });
    }
}

async fn handle_client(state: Arc<State>, addr: SocketAddr, conn: TcpStream) -> anyhow::Result<()> {
    let mut stream = Framed::new(conn, LinesCodec::new());
    stream.send("Your username:").await?;

    let username = match stream.next().await {
        Some(Ok(name)) => name,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    let mut peer = state.add_peer(addr, username, stream).await;
    let message = Arc::new(Message::user_joined(&peer.username));
    info!("{}", message);
    state.broadcast(addr, message).await;

    while let Some(content) = peer.receiver.next().await {
        let line = match content {
            Ok(c) => c,
            Err(e) => {
                warn!("recv msg from {}: err: {}", addr, e);
                break;
            }
        };

        let msg = Arc::new(Message::chat(&peer.username, line));
        state.broadcast(addr, msg).await;
    }

    state.peers.remove(&addr);

    let left_msg = Arc::new(Message::user_left(&peer.username));
    state.broadcast(addr, left_msg).await;

    anyhow::Ok(())
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

impl Message {
    fn user_joined(name: &str) -> Self {
        Self::UserJoined(format!("{} joined the chat", name))
    }

    fn user_left(name: &str) -> Self {
        Self::UserLeft(format!("{} leave the chat", name))
    }

    fn chat(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserJoined(name) => write!(f, "[{}]", name),
            Self::UserLeft(name) => write!(f, "user: {} left", name),
            Self::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}
