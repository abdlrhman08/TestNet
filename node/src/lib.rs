use futures::StreamExt;

use bollard::{
    container::{Config, CreateContainerOptions},
    errors::Error,
    image::CreateImageOptions,
    secret::HostConfig,
    Docker,
};

pub mod tester;

type ContainerOpts = Option<CreateContainerOptions<String>>;

pub struct ContainerManager<'a> {
    docker: &'a Docker,
}

impl<'a> ContainerManager<'a> {
    pub fn new(docker: &'a Docker) -> Self {
        Self {
            docker,
        }
    }

    pub async fn start_container(&mut self, image: &str, host_config: &HostConfig) -> Result<String, Error> {
        let container_config = Config {
            image: Some(image),
            cmd: Some(vec!["tail", "-f", "/dev/null"]),
            host_config: Some(host_config.clone()),
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
                    Ok(container.unwrap().id)
                } else {
                    Err(Error::DockerResponseServerError {status_code, message})
                }
            }
            Err(error) => Err(error),
            Ok(response) => Ok(response.id),
        }
    }
}
