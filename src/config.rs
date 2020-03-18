use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChatConfig {
    pub secret: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemConfig {
    pub server: ServerConfig,
    pub line_chat: LineChatConfig,
}

impl SystemConfig {
    fn new() -> SystemConfig {
        let f = File::open("./config.yml").expect("can read the config file");
        let reader = BufReader::new(f);

        let contents: SystemConfig = from_reader(reader).expect("not a system config");
        contents
    }
}

lazy_static! {
    pub static ref SYSTEM_CONFIG: SystemConfig = { SystemConfig::new() };
}

//pub fn print_config() {
//println!("{:?}", parse_config());
//}
