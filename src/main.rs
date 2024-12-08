use yaml_rust2::YamlLoader;
use std::fs;
use std::io::prelude::*;

// Use a struct here?
struct Pipeline {
    image: String,
    command: String,
}

fn main() {

    let path = "assets/file.yaml";
    let pipeline = build_pipeline(path);
    println!("{}", pipeline.image);
    println!("{}", pipeline.command);
}

fn build_pipeline(path: &str) -> Pipeline {
    let mut file = fs::File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    let image = doc["image"].as_str().unwrap();
    let command = doc["command"].as_str().unwrap();
    
    Pipeline {
        image: image.to_string(),
        command: command.to_string(),
    }
}
