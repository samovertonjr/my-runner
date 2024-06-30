use yaml_rust2::{YamlLoader, YamlEmitter};

fn main() {
    let s = read_file();
    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Array/map-like accesses are checked and won't panic.
    // They will return `BadValue` if the access is invalid.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}

use std::fs::File;
use std::io::prelude::*;

fn read_file() -> str {
    let mut file = File::open("./assets/file.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}", contents);
    Ok(())
}
