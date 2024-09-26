use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    response::Response
};

pub async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    //probable move the scoket to the server or something
    unimplemented!();
}
