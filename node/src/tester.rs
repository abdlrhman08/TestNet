use std::fs::File;
use futures::StreamExt;

use bollard::exec::{CreateExecOptions, CreateExecResults, StartExecResults};
use bollard::Docker;
use serde::{Serialize, Deserialize};

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

pub struct PipelineRunner<'a> {
    docker: &'a Docker
}

impl<'a> PipelineRunner<'a> {
    pub fn new(docker: &'a Docker) -> Self {
        Self { docker } 
    }

    // The reference here for docker maybe changed for a global variable
    pub async fn create_pipeline(&self, container_id: &str) -> Result<Vec<StageWithExec>, Box<dyn std::error::Error>> {
        let yaml_file = File::open("./TestNet.yaml")?;
        let pipeline_file: Pipeline = serde_yaml::from_reader(yaml_file)?;
        let mut pipeline_stages = Vec::new();

        for Stage { name, commands } in pipeline_file.stages {
            let mut exec_commands = Vec::new();
            for command in commands {
                let splitted_command: Vec<String> = command.split_whitespace().map(|cmd| cmd.to_owned()).collect();
                let exec_option = CreateExecOptions {
                    cmd: Some(splitted_command),
                    attach_stdout: Some(true),
                    ..Default::default()
                };

                let created_exec = self.docker
                    .create_exec(container_id, exec_option)
                    .await
                    .unwrap();

                exec_commands.push(created_exec);
            }
            pipeline_stages.push(StageWithExec {
                name,
                commands: exec_commands
            });
        }

        Ok(pipeline_stages)
    }

    pub async fn run_pipeline(&self, container_id: &str) {
        let pipeline_stages = self.import_pipeline(container_id).await.unwrap();
        
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
                    }
                }

                let exec_result = self.docker.inspect_exec(&command.id).await.unwrap();
                println!("last command exit code {}", exec_result.exit_code.unwrap());
            }
        }
    }
}



