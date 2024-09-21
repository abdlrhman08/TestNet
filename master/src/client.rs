use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Json, Router,
};
use net_interface::{interface::Job, JobQueue};
use tokio::sync::{Mutex, Notify};
use tower_http::services::ServeDir;

mod response;

#[derive(Clone)]
pub struct ServerConfig {
    pub job_queue: Arc<Mutex<JobQueue>>,
    pub notifier: Arc<Notify>,
}

async fn get_root() -> Html<String> {
    response::html_response("index.html")
}

async fn project_trigger(
    State(config): State<ServerConfig>,
    Path(project_name): Path<String>,
    Json(payload): Json<crate::Trigger>,
) {
    //TODO!: save the registered project with some of its data in a
    // key value db
    let new_job = Job {
        project_name: payload.repository.name,
        git_url: payload.repository.clone_url,
    };

    let job_queue = &mut *config.job_queue.lock().await;
    job_queue.queue_job(new_job);
    // I don't think its healty to start the scheduler every time
    // we get a job
    config.notifier.notify_one();
}

pub async fn start_server(config: ServerConfig, port: u16) {
    let static_server = ServeDir::new("dist/assets/");
    let app = Router::new()
        .route("/", get(get_root))
        .route("/hooks/:project_name", post(project_trigger))
        .nest_service("/assets/", static_server)
        .with_state(config);
    let addr = format!("127.0.0.1:{}", port);

    println!("Starting server on port {}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
