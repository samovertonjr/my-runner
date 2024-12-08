use yaml_rust2::YamlLoader;
use std::fs;
use std::io::prelude::*;

struct Pipeline {
    image: String,
    command: String,
}

fn main() {

    let path = "assets/file.yml";
    let pipeline = build_pipeline(path);
    println!("You are using image   --> {}", pipeline.image);
    println!("You are using command --> {}", pipeline.command);
}

fn build_pipeline(path: &str) -> Pipeline {
    let mut file = fs::File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    // dbg!(doc);
    let image = doc["image"].as_str().unwrap();
    let command = doc["command"].as_str().unwrap();
    // Todo:
    // Create branching logic between gitlab, github, etc.
    // define jobs in struct
    // allow for multiple jobs in struct
    // allow for different job definitions between gitlab and github 
    // Start with gitlab
    Pipeline {
        image: image.to_string(),
        command: command.to_string(),
    }
}
