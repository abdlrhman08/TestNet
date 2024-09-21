use futures::StreamExt;

use std::process::Command;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    errors::Error,
    image::CreateImageOptions,
    secret::HostConfig,
    Docker,
};

pub mod tester;

type ContainerOpts = Option<CreateContainerOptions<String>>;

pub struct ContainerManager<'a> {
    docker: &'a Docker,
    host_config: HostConfig
}

impl<'a> ContainerManager<'a> {
    pub fn new(docker: &'a Docker, repos_path: &str) -> Self {
        let host_config = HostConfig {
            binds: Some(vec![format!("{}:/tmp", repos_path)]),
            ..Default::default()
        };

        Self {
            docker,
            host_config
        }
    }

    pub async fn start_container(&mut self, image: &str) -> Result<String, Error> {
        let container_config = Config {
            image: Some(image),

            // to prevent the container from stopping
            cmd: Some(vec!["tail", "-f", "/dev/null"]),
            host_config: Some(self.host_config.clone()),
            ..Default::default()
        };
        let mut container = self
            .docker
            .create_container(ContainerOpts::None, container_config.clone())
            .await;

        match container {
            Err(Error::DockerResponseServerError { status_code, message }) => {
                if status_code == 404 {
                    let image = CreateImageOptions {
                        from_image: image,
                        ..Default::default()
                    };
                    let mut image_stream = self.docker.create_image(Some(image), None, None);
                    while let Some(data) = image_stream.next().await {
                        let info = data.unwrap();
                        println!("Downloading {:?}, {:?}", info.id, info.progress);
                    }
                    container = self
                        .docker
                        .create_container(ContainerOpts::None, container_config)
                        .await;

                    let container = container.unwrap();
                    self.docker.start_container(&container.id, None::<StartContainerOptions<String>>).await?;
                    Ok(container.id)
                } else {
                    Err(Error::DockerResponseServerError {status_code, message})
                }
            }
            Err(error) => Err(error),
            Ok(response) => {
                self.docker.start_container(&response.id, None::<StartContainerOptions<String>>).await?;
                Ok(response.id)
            },
        }
    }
}

pub fn clone_repo(repo: &str) {
    let clone_command = Command::new("git")
         .arg("clone")
         .arg(repo)
         .spawn()
         .unwrap();

    // save the output in a log struct
    clone_command.wait_with_output();
}


