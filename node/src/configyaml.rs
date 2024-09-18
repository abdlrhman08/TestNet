use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Stage {
    name: String,
    commands: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    image: String,
    stages: Vec<Stage>
}


