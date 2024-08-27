use axum::{
    extract::Path,
    response::Html,
    routing::{get, post},
    Json, Router,
};

use crate::queue;

mod response;

async fn get_root() -> Html<String> {
    response::html_response("index.html")
}

async fn register_project(
    Path(project_name): Path<String>,
    Json(payload): Json<crate::queue::Trigger>,
) {
    println!("Registered {}", project_name);
    println!("Registered {:?}", payload);
}

pub async fn start_server() {
    let message_queue = queue::JobQueue::new();

    let app = Router::new()
        .route("/", get(get_root))
        .route("/hooks/:project_name", post(register_project));

    println!("Starting server on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
