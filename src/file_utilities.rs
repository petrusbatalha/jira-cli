extern crate yaml_rust;

use self::yaml_rust::{Yaml, YamlLoader};
use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};

pub async fn load_yaml(yaml_path: &str) -> Result<String, anyhow::Error> {
    // Open stories yaml.
    let mut yaml_file = File::open(yaml_path).unwrap();

    let mut yaml_as_string = String::new();

    match yaml_file.read_to_string(&mut yaml_as_string) {
        Ok(_) => Ok(yaml_as_string),
        Err(_) => Err(anyhow!("failed to read yaml")),
    }
}

pub async fn json_to_file<T: Serialize>(payload: T, path: &str) -> serde_json::Result<()> {
    let file = File::create(&path).unwrap();

    serde_json::to_writer(file, &payload)
}

pub async fn json_from_file<T: for<'de> Deserialize<'de>>(
    path: &str,
) -> Result<serde_json::Result<T>, anyhow::Error> {
    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader::<BufReader<File>, T>(reader) {
                Ok(json) => Ok(Ok(json)),
                Err(e) => bail!(e),
            }
        }
        Err(e) => bail!(e),
    }
}
