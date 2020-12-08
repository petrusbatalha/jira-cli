extern crate yaml_rust;

use self::yaml_rust::{Yaml, YamlLoader};
use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::HashMap;
use serde::{Serialize, Deserialize, de};
use serde::de::value::Error;

pub async fn load_yaml(yaml_path: &str) -> Yaml {
    // Open stories yaml.
    let mut yaml_file = File::open(yaml_path).unwrap();

    let mut yaml_as_string = String::new();

    yaml_file
        .read_to_string(&mut yaml_as_string)
        .expect("Failed to load yaml");

    let yaml_file = YamlLoader::load_from_str(&yaml_as_string).unwrap();
    yaml_file[0].clone()
}

pub async fn json_to_file<T: Serialize>(payload: T, path: &str) -> serde_json::Result<()> {
    let file = File::create(&path).unwrap();

    serde_json::to_writer(file, &payload)
}

pub async fn json_from_file<T: de::DeserializeOwned, Deserialize>(path: &str) -> serde_json::Result<T> {
    let file = File::open(&path).unwrap();

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
}