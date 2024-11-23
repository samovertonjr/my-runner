use yaml_rust2::YamlLoader;
use std::fs;
use std::io::prelude::*;

fn main() {

    let path = "assets/file.yaml";
    let image = get_image_name(path);
    println!("You are about to run image --> {image}");
}

fn get_image_name(path: &str) -> String {
    let mut file = fs::File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let image_name = doc["image"].as_str().unwrap();
    return image_name.to_string();
}
