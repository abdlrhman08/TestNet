use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    http::header::HeaderValue,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use net_interface::{interface::Job, JobQueue};
use tokio::sync::{Mutex, Notify};
use tower_http::services::ServeDir;

use crate::Project;

mod response;
mod logstream;

// There is a lot of arcs !!!

#[derive(Clone)]
pub struct ServerConfig {
    pub job_queue: Arc<Mutex<JobQueue>>,
    pub notifier: Arc<Notify>,
    
    //This must be a reference for it work properly
    pub projects: Arc<Mutex<HashMap<u64, Project>>>
}

async fn get_root() -> Html<String> {
    response::html_response("index.html")
}

async fn list_projects(State(config): State<ServerConfig>) -> Json<HashMap<u64, Project>> { 
    let projects =  & *config.projects.lock().await;
    Json(projects.clone())
}

async fn project_trigger(
    header: HeaderMap,
    State(config): State<ServerConfig>,
    Path(project_name): Path<String>,
    Json(payload): Json<crate::Trigger>,
) {
    if let Some(request_type) = header.get("X-GitHub-Event") {
        match request_type.to_str() {
            Ok("ping") => {
                let new_project = Project {
                    id: 122, // this better be the initial hook id
                    name: project_name,
                    full_name: payload.repository.full_name,
                    url: payload.repository.url,
                    clone_url: payload.repository.clone_url,
                    stages: None
                };
                let project_map = &mut config.projects.lock().await;
                project_map.insert(122, new_project);
            },
            Ok(other) => {
                let new_job = Job {
                    project_name: payload.repository.name,
                    git_url: payload.repository.clone_url,
                };

                let job_queue = &mut *config.job_queue.lock().await;
                job_queue.queue_job(new_job);
                // I don't think its healty to start the scheduler every time
                // we get a job
                config.notifier.notify_one();
            },
            Err(_) => println!("Unknown request type")
        }
    } else {
        println!("Request not from none service");
    }
}

pub async fn start_server<'a>(config: ServerConfig, port: u16) {
    let static_server = ServeDir::new("dist/assets/");
    
    let api_router = Router::new().route("/projects", get(list_projects));
    let app = Router::new()
        .route("/", get(get_root))
        .route("/hooks/:project_name", post(project_trigger))
        .route("/ws", get(logstream::handler))
        .nest_service("/assets/", static_server)
        .nest("/api", api_router)
        .with_state(config);


    let addr = format!("127.0.0.1:{}", port);

    println!("Starting server on port {}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
