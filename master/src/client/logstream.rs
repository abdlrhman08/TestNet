use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    response::Response
};


pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    // probable move the scoket to the server or something
    let ws = &mut *net_interface::LOG_STREAMER.lock().await;
    if let None = *ws {
        *ws = Some(socket);
    } else {
        eprintln!("Socket is already connected");
    }
}
