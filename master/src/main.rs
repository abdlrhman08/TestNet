use tokio::task;
use tonic::transport::Server;

use master::client;
use net_interface::interface::test_net_server;
use net_interface::TestNetServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:5001".parse()?;
    let grpc_server = TestNetServer::new();

    // start the client interface on another thread since it is not the main
    // objective
    let client_thread = task::spawn(async {
        client::start_server().await;
    });

    Server::builder()
        .add_service(test_net_server::TestNetServer::new(grpc_server))
        .serve(address)
        .await?;

    Ok(())
}
