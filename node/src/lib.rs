use futures::StreamExt;
use std::collections::HashMap;

use bollard::{
    container::{Config, CreateContainerOptions},
    errors::Error,
    image::CreateImageOptions,
    secret::HostConfig,
    Docker,
};

pub struct ContainerCache<'a> {
    docker: &'a Docker,
    container_map: HashMap<String, String>,
}

impl<'a> ContainerCache<'a> {
    pub fn new(docker: &'a Docker) -> Self {
        Self {
            docker,
            container_map: HashMap::new(),
        }
    }

    pub async fn get_container(&mut self, image: &str, host_config: &HostConfig) -> &str {
        if let Some(container_id) = self.container_map.get_mut(image) {
            return container_id.as_str();
        }

        let container_config = Config {
            image: Some("ubuntu"),
            host_config: Some(host_config.clone()),
            ..Default::default()
        };
        let container = self
            .docker
            .create_container(None::<CreateContainerOptions<String>>, container_config)
            .await;

        if let Err(Error::DockerResponseServerError {
            status_code,
            message: _,
        }) = container
        {
            if status_code == 404 {
                let image = CreateImageOptions {
                    from_image: image,
                    ..Default::default()
                };
                let mut image_stream = self.docker.create_image(Some(image), None, None);
                while let Some(data) = image_stream.next().await {
                    println!("Downloading {:?}", data);
                }
            }
        }

        "ubuntu"
    }
}
