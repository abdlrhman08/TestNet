use net_interface::interface::test_net_client::TestNetClient;
use net_interface::interface::Empty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = TestNetClient::connect("http://127.0.0.1:5020").await?;
    let mut job_receiver = master.register(Empty {}).await?.into_inner();

    while let Some(job) = job_receiver.message().await? {
        // every time we get a job we start execution and send the logs
        // then we wait for another
        println!("Received: {}", job.repo_url);
        //TODO!: job execution
    }

    Ok(())
}
