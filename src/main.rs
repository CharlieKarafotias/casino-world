mod bet;
mod crapless;
mod game;
mod player;

use crate::game::{GameNames, GameProvider};
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn, LevelFilter};
use std::sync::Arc;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

struct GameProviders {
    crapless: GameProvider,
}

impl GameProviders {
    fn new() -> Self {
        GameProviders {
            crapless: GameProvider::new(GameNames::Crapless, 1),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    let game_providers = Arc::new(GameProviders::new());

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, game_providers.clone()));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, game_providers: Arc<GameProviders>) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    let player = Arc::new(player::Player::new(addr.to_string(), 100));
    info!("New Player created: {:?}", player);

    // Listen for new messages
    read.for_each(|message| {
        let game_providers = Arc::clone(&game_providers);
        let player = Arc::clone(&player);

        // TODO: refactor
        // What if game providers only handles the availability of games?
        // let each game thread control when it should live/die, player connect/disconnect, etc
        // when event happens, only that game is affected, rest of the games are unaffected
        async move {
            match message {
                Ok(msg) => {
                    match msg {
                        Message::Text(text) => {
                            info!("Received a text message from {addr}: {text}");
                        }
                        Message::Close(..) => {
                            info!("The client: {addr} has closed the connection");
                        }
                        _ => {
                            warn!("Received a non-text message from {addr}: {msg:?}");
                        }
                    }
                }
                Err(e) => error!("Failed to receive a message: {e}"),
            }
        }
    })
    .await;
}
