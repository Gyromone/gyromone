use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Foo {
    bar: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfig {
    pub foo: Foo,
}

pub fn parse_config() -> Result<SystemConfig, String> {
    let f = File::open("./config.yml").expect("can read the config file");
    let reader = BufReader::new(f);

    let contents: SystemConfig = from_reader(reader).expect("not a system config");
    Ok(contents)
}

pub fn print_config() {
    match parse_config() {
        Ok(config) => println!("{:?}", config),
        Err(s) => println!("{}", s),
    }
}
