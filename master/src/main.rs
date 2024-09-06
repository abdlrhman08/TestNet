use tokio::task;
use tonic::transport::Server;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use master::client;
use net_interface::interface::{test_net_server, Job};
use net_interface::{JobQueue, TestNetServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job_queue = Arc::new(Mutex::new(JobQueue::new()));

    let address = "127.0.0.1:5001".parse()?;
    let grpc_server = TestNetServer::new(Arc::clone(&job_queue));

    // start the client interface on another thread since it is not the main
    // objective
    let client_thread = task::spawn(async {
        client::start_server().await;
    });

    let cloned_queue = Arc::clone(&job_queue);
    let push_job_thread = std::thread::spawn(move || {
        let mut t = 0;
        loop {
            println!("thread running");
            t += 1;
            let job_queue = &mut *cloned_queue.lock().unwrap();
            let job = Job {
                payload: format!("Job number {}", t),
            };
            job_queue.queue_job(job);
            std::thread::sleep(Duration::from_secs(5));
        }
    });

    Server::builder()
        .add_service(test_net_server::TestNetServer::new(grpc_server))
        .serve(address)
        .await?;

    Ok(())
}
