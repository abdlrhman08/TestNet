use std::fs::File;

use bollard::exec::CreateExecOptions;
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
struct StageWithExec {
    name: String,
    commands: Vec<CreateExecOptions<String>>,
}

pub fn import_pipeline() -> Result<Pipeline, Box<dyn std::error::Error>>{
    let yaml_file = File::open("./TestNet.yaml")?;
    let pipeline_file: Pipeline = serde_yaml::from_reader(yaml_file)?;

    let exec_commands = Vec::new();
    for Stage { name, commands } in pipeline_file.stages {
        for command in commands {
            let splitted_command: Vec<String> = command.split_whitespace().collect();
            exec_commands.push(CreateExecOptions {
                cmd: Some(splitted_command),
                attach_stdout: Some(true),
                ..Default::default()
            });
        } 
    }

    Ok(pipeline_file)
}
