use master::client;

#[tokio::main]
async fn main() {
    client::start_server().await;
}
