mod bet;
mod crapless;
mod game;
mod player;

use crate::game::{GameNames, GameProvider};
use futures_util::{SinkExt, StreamExt};
use log::{info, warn, LevelFilter};
use std::sync::Arc;
use std::{env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex as TokioMutex;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

struct GameProviders {
    crapless: RwLock<GameProvider>,
}

impl GameProviders {
    fn new() -> Self {
        GameProviders {
            crapless: RwLock::new(GameProvider::new(GameNames::Crapless, 1)),
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
        tokio::spawn(accept_connection(stream, Arc::clone(&game_providers)));
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
    let write = Arc::new(TokioMutex::new(write));

    let player = Arc::new(RwLock::new(player::Player::new(addr.to_string(), 100)));
    info!("New Player created: {:?}", player);

    // Listen for new messages
    read.for_each(|message| {
        let write = Arc::clone(&write);
        let game_providers = Arc::clone(&game_providers);
        let player = Arc::clone(&player);

        // TODO: refactor
        // What if game providers only handles the availability of games?
        // let each game thread control when it should live/die, player connect/disconnect, etc
        // when event happens, only that game is affected, rest of the games are unaffected
        async move {
            if let Ok(msg) = message {
                if msg.is_text() {
                    let text = msg.to_text().expect("Failed to convert message to text");
                    match text {
                        "Game: Crapless" => {
                            if let Ok(mut provider) = game_providers.crapless.try_write() {
                                // TODO: when awaiting here, it blocks whole server
                                provider.add_player_to_game(Arc::clone(&player)).await;
                                let mut write_guard = write.lock().await;
                                if let Err(e) = write_guard
                                    .send(Message::text("Welcome to the Crapless game!"))
                                    .await
                                {
                                    warn!("Failed to send welcome message: {}", e);
                                }
                            }
                        }
                        _ => warn!("Received unknown message: {text}"),
                    }
                }
                if msg.is_close() {
                    info!("The client has closed the connection");
                    if let Ok(mut provider) = game_providers.crapless.try_write() {
                        provider.remove_player_from_game(Arc::clone(&player)).await;
                    }
                    info!(
                        "Updated game providers: {:#?}",
                        game_providers.crapless.read().await.game_count()
                    );
                }
            } else {
                warn!("Failed to read message");
            }
        }
    })
    .await;
}
