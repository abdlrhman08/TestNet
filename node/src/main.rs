use std::process::Command;

use net_interface::interface::test_net_client::TestNetClient;
use net_interface::interface::Empty;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    exec::CreateExecOptions,
    secret::HostConfig,
    Docker,
};

//TODO!: figure out better logging than println!

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = TestNetClient::connect("http://127.0.0.1:5001").await?;
    let mut job_receiver = master.register(Empty {}).await?.into_inner();
    let repos_path = "/home/tmp/repos";

    let docker = Docker::connect_with_defaults().expect("Failed to connect to the docker daemon, please ensure that docker is installed and running as a service");
    let mut container_cache = node::ContainerCache::new(&docker);
    std::fs::create_dir_all(repos_path)?;
    std::env::set_current_dir(repos_path)?;

    let host_config = HostConfig {
        binds: Some(vec![format!("{}:/tmp", repos_path)]),
        ..Default::default()
    };

    while let Some(job) = job_receiver.message().await? {
        // pre-test setup
        let current_container = container_cache
            .get_container("hello-world", &host_config)
            .await;
        docker
            .start_container(current_container, None::<StartContainerOptions<String>>)
            .await
            .unwrap();

        println!("Received: {}", job.git_url);
        println!("Cloning");

        // let clone_command = Command::new("git")
        //     .arg("clone")
        //     .arg(job.git_url)
        //     .spawn()
        //     .unwrap();
        // clone_command.wait_with_output();

        //start test
        let command = CreateExecOptions {
            cmd: Some(vec!["echo", "hello"]),
            attach_stdout: Some(true),
            ..Default::default()
        };
        docker.create_exec(current_container, command).await;

        //TODO!: cleanup
    }

    Ok(())
}
