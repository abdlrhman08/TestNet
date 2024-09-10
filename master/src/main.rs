use clap::Parser;
use std::sync::Arc;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::{
    sync::{Mutex, Notify},
    task,
};
use tonic::transport::Server;

use master::{
    client::{self, ServerConfig},
    scheduler::Scheduler,
};
use net_interface::interface::test_net_server;
use net_interface::{JobQueue, NodeData, TestNetServer};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 25600)]
    port: u16,

    #[arg(short, long, default_value_t = 5001)]
    rpc_port: u16,
}

// This code is getting messy with alot of arcs, I'll probably
// have to refactor alot after I get to a working stage

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let node_map: HashMap<String, NodeData> = HashMap::new();

    let job_queue = Arc::new(Mutex::new(JobQueue::new()));
    let node_data = Arc::new(Mutex::new(node_map));

    let grpc_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), args.rpc_port);
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

    println!("Starting grpc server on port {}", args.port);
    Server::builder()
        .add_service(test_net_server::TestNetServer::new(grpc_server))
        .serve(grpc_address)
        .await?;

    Ok(())
}
