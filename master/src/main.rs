use tokio::sync::{Mutex, Notify};
use tokio::task;
use tonic::transport::Server;

use master::{
    client::{self, ServerConfig},
    scheduler::Scheduler,
};
use net_interface::interface::test_net_server;
use net_interface::{JobQueue, NodeData, TestNetServer};
use std::collections::HashMap;
use std::sync::Arc;

//This code is getting messy with alot of arcs, I am trying
// my bes

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node_map: HashMap<String, NodeData> = HashMap::new();

    let job_queue = Arc::new(Mutex::new(JobQueue::new()));
    let node_data = Arc::new(Mutex::new(node_map));

    let address = "127.0.0.1:5020".parse()?;
    let scheduler = Scheduler::new(&job_queue, &node_data);
    let scheduler_notifier = Arc::new(Notify::new());
    let grpc_server = TestNetServer::new(&job_queue, &node_data);

    let server_config = ServerConfig {
        job_queue: Arc::clone(&job_queue),
        notifier: Arc::clone(&scheduler_notifier),
    };

    // start the client interface on another thread since it is not the main
    // objective
    let _client_thread = task::spawn(async {
        client::start_server(server_config).await;
    });
    let _scheduler_task = task::spawn(async move {
        loop {
            scheduler.schedule_jobs().await;
            scheduler_notifier.notified().await;
        }
    });

    Server::builder()
        .add_service(test_net_server::TestNetServer::new(grpc_server))
        .serve(address)
        .await?;

    Ok(())
}
