use net_interface::interface::test_net_client::TestNetClient;
use net_interface::interface::Empty;

use node::tester::PipelineRunner;
use node::ContainerManager;

use bollard::Docker;

//TODO!: figure out better logging than println!

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut master = TestNetClient::connect("http://127.0.0.1:5001").await?;
    let mut job_receiver = master.register(Empty {}).await?.into_inner();
    let repos_path = "/home/tmp/repos";
    let docker = Docker::connect_with_defaults().expect("Failed to connect to the docker daemon, please ensure that docker is installed and running as a service");

    let container_manager = ContainerManager::new(&docker, repos_path);
    let mut tester = PipelineRunner::new(&docker, &mut master, container_manager);
    
    std::fs::create_dir_all(repos_path)?;
    std::env::set_current_dir(repos_path)?;

    while let Some(job) = job_receiver.message().await? {
        println!("Received a job");
        println!("Received: {}", job.git_url);
        println!("Cloning");
        
        // pretest
        node::clone_repo(&job.git_url);
        
        //test
        tester.run_pipeline(job).await;

        //cleanup
        tester.clean().await?;
    }

    Ok(())
}
