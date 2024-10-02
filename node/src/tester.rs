use std::fs::File;
use futures::StreamExt;

use net_interface::interface::Job;
use net_interface::interface::LogObject;
use net_interface::interface::Stages;
use net_interface::interface::test_net_client::TestNetClient;

use tonic::Request;
use tonic::transport::Channel;


use bollard::exec::{CreateExecOptions, CreateExecResults, StartExecResults};
use bollard::container::{KillContainerOptions, RemoveContainerOptions, LogOutput};
use bollard::Docker;
use serde::{Serialize, Deserialize};

use crate::ContainerManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    name: String,
    commands: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    image: String,
    stages: Vec<Stage>
}

// This may change
pub struct StageWithExec {
    pub name: String,
    pub commands: Vec<CreateExecResults>,
}

pub trait Extract {
    // for now it will return only a struct, later it will
    // return a whole log object specifying the stream type
    fn get_data(self) -> String;
}

impl Extract for LogOutput {
    fn get_data(self) -> String {
        match self {
            LogOutput::StdOut { message } => String::from_utf8(message.to_vec()).unwrap(),
            LogOutput::StdIn { message } => String::from_utf8(message.to_vec()).unwrap(),
            LogOutput::StdErr { message } => String::from_utf8(message.to_vec()).unwrap(),
            LogOutput::Console { message } => String::from_utf8(message.to_vec()).unwrap(),
        }
    }
}

// Later this may need a mutex instead of a mutable reference
pub struct PipelineRunner<'a> {
    docker: &'a Docker,
    master: &'a mut TestNetClient<Channel>,
    container_manager: ContainerManager<'a>,
    current_container: Option<String>
}

impl<'a> PipelineRunner<'a> {
    pub fn new(
        docker: &'a Docker,
        master: &'a mut TestNetClient<Channel>,
        container_manager: ContainerManager<'a>
    ) -> Self {
        Self { 
            docker,
            master,
            container_manager,
            current_container: None
        } 
    }

    // The reference here for docker maybe changed for a global variable
    pub async fn create_pipeline(&mut self, job: Job) -> Result<Vec<StageWithExec>, Box<dyn std::error::Error>> {
        println!("{}", job.project_name);
        let yaml_file = File::open(format!("./{}/TestNet.yaml", &job.project_name))?;
        let pipeline_file: Pipeline = serde_yaml::from_reader(yaml_file)?;
        let working_dir = format!("/tmp/{}", &job.project_name);

        let container = self.container_manager
            .start_container(&pipeline_file.image)
            .await?;

        let mut pipeline_stages = Vec::new();
        let mut stage_names = Vec::new();

        for Stage { name, commands } in pipeline_file.stages {
            let mut exec_commands = Vec::new();
            for command in commands {
                let splitted_command: Vec<String> = command.split_whitespace().map(|cmd| cmd.to_owned()).collect();
                let exec_option = CreateExecOptions {
                    working_dir: Some(working_dir.clone()),
                    cmd: Some(splitted_command),
                    attach_stdout: Some(true),
                    ..Default::default()
                };

                let created_exec = self.docker
                    .create_exec(&container, exec_option)
                    .await
                    .unwrap();

                exec_commands.push(created_exec);
            }
            stage_names.push(name.clone());
            pipeline_stages.push(StageWithExec {
                name,
                commands: exec_commands
            });
        }

        let request = tonic::Request::new(Stages {
            stages: stage_names
        });
        self.master.register_stages(request).await;
        self.current_container = Some(container);
        Ok(pipeline_stages)
    }

    //It is better to move the host config as a struct variable
    pub async fn run_pipeline(&mut self, job: Job) {
        let pipeline_stages = self.create_pipeline(job).await.unwrap();
        
        for stage in pipeline_stages.iter() {
            for command in stage.commands.iter() {
                let start_exec = self.docker
                    .start_exec(&command.id, None)
                    .await
                    .unwrap();


                if let StartExecResults::Attached { mut output, .. } = start_exec {
                    while let Some(Ok(out)) = output.next().await {
                        //TODO!: stream the output to the user
                        println!("{} from container", out);
                        let log_obj = Request::new(LogObject {
                            job_id: "asd".to_string(),
                            stage: stage.name.clone(),
                            log: out.get_data()
                        });
                        self.master.send_log(log_obj).await;
                    }
                }

                let exec_result = self.docker.inspect_exec(&command.id).await.unwrap();
                println!("last command exit code {}", exec_result.exit_code.unwrap());
            }
        }
    }

    pub async fn clean(&mut self) -> Result<(), &str>{
        match self.current_container {
            None => Err("There is no any containers currently running"),
            Some(ref container_id) => {
                self.docker.kill_container(container_id, None::<KillContainerOptions<String>>).await;
                self.docker.remove_container(container_id, None::<RemoveContainerOptions>).await;
                self.current_container = None;
                Ok(())
            }
        }
    }
}



