use std::fs::DirBuilder;
use std::process::Command;

use net_interface::interface::test_net_client::TestNetClient;
use net_interface::interface::Empty;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    secret::HostConfig,
    Docker,
};

//TODO!: figure out better logging than println!

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = TestNetClient::connect("http://127.0.0.1:5020").await?;
    let mut job_receiver = master.register(Empty {}).await?.into_inner();
    let repos_path = "/home/tmp/repos";

    let docker = Docker::connect_with_defaults().expect("Failed to connect to the docker daemon, please ensure that docker is installed and running as a service");
    let _directory_creator = DirBuilder::new()
        .recursive(true)
        .create(repos_path)
        .unwrap();
    std::env::set_current_dir(repos_path);

    let host_config = HostConfig {
        binds: Some(vec![format!("{}:/tmp", repos_path)]),
        ..Default::default()
    };

    while let Some(job) = job_receiver.message().await? {
        // pre-test setup
        let container_config = Config {
            image: Some("ubuntu"),
            host_config: Some(host_config.clone()),
            ..Default::default()
        };
        let container = docker
            .create_container(None::<CreateContainerOptions<String>>, container_config)
            .await
            .unwrap();

        docker
            .start_container(container.id.as_str(), None::<StartContainerOptions<String>>)
            .await
            .unwrap();

        println!("Received: {}", job.git_url);
        println!("Cloning");

        let clone_command = Command::new("git")
            .arg("clone")
            .arg(job.git_url)
            .spawn()
            .unwrap();
        clone_command.wait_with_output();

        //start test
    }

    Ok(())
}
