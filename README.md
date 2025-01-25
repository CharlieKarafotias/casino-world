# Websocket Server

This is a basic websocket server written in Rust. It uses the [tokio](https://github.com/tokio-rs/tokio) framework for async IO and the [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) library for websocket support.

## Running the Server

To run the server, first build the project with `cargo run`. This will start the server in Debug mode.

To connect to the server using Postman, use the following settings:
- URL: `ws://127.0.0.1:8080`
- Protocol: `websocket`

### Current protocol support

- Crapless Craps Game
  - Initiate with `Game: Crapless`
