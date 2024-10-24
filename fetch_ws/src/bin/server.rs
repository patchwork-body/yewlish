use futures::SinkExt;
use futures::StreamExt;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_connection));

    // Run the warp server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("WebSocket server is running at ws://{addr}");

    warp::serve(routes).run(addr).await;
}

// Handles each WebSocket connection
async fn handle_connection(websocket: warp::ws::WebSocket) {
    let (mut tx, mut rx) = websocket.split();

    // Create a channel to simulate communication
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    // Spawn a task to handle incoming messages from WebSocket
    tokio::spawn(async move {
        while let Some(Ok(msg)) = rx.next().await {
            if msg.is_text() || msg.is_binary() {
                // Forward message to the sender
                msg_tx.send(msg).unwrap();
            }
        }
    });

    // Spawn a task to handle outgoing messages to WebSocket
    tokio::spawn(async move {
        while let Some(msg) = msg_rx.recv().await {
            if tx.send(msg).await.is_err() {
                break;
            }
        }
    });
}
