use net_interface::interface::test_net_client::TestNetClient;
use net_interface::interface::Sentinel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = TestNetClient::connect("http://127.0.0.1:5001").await?;
    let request = tonic::Request::new(Sentinel { d: "1".to_string() });

    let response = master.pull_job(request).await?;
    println!("{}", response.into_inner().payload);

    Ok(())
}
