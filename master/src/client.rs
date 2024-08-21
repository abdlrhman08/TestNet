use axum::{response::Html, routing::get, Router};

mod response;

async fn get_root() -> Html<String> {
    response::html_response("index.html")
}

pub async fn start_server() {
    let app = Router::new().route("/", get(get_root));

    println!("Starting server on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
