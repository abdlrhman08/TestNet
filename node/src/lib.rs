use futures::StreamExt;
use std::collections::HashMap;

use bollard::{
    container::{Config, CreateContainerOptions},
    errors::Error,
    image::CreateImageOptions,
    secret::HostConfig,
    Docker,
};

mod configyaml;

type ContainerOpts = Option<CreateContainerOptions<String>>;

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

    pub async fn get_container(&mut self, image: &str, host_config: &HostConfig) -> String {
        if let Some(container_id) = self.container_map.get(image) {
            return container_id.to_string();
        }

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
                    let info = data.unwrap();
                    println!("Downloading {:?}, {:?}", info.id, info.progress);
                }
                container = self
                    .docker
                    .create_container(ContainerOpts::None, container_config)
                    .await;
            }
        }
        // I will figure a better way later
        let container_id = container.unwrap().id.clone();
        self.container_map
            .insert(image.to_owned(), container_id.clone());
        container_id
    }
}
