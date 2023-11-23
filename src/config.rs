use std::fs::File;
use std::path::Path;
use iroha_config::client::Configuration;

pub fn get_config() -> Configuration {
        let path = Path::new("C:\\RustProjects\\iroha_stable\\src\\resources\\config.json");
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).expect(&format!("Failed to read config at {:?}", &path))
}