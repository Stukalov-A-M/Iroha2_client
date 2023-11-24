use iroha_config::client::Configuration;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe();
    let binding = exe_path?;
    let ancestors = binding.ancestors();
    for ancestor in ancestors {
        if ancestor.file_name() == Some("target".as_ref()) {
            let source_path = Path::new(ancestor);
            let mut config_path = PathBuf::from(source_path.parent().unwrap());
            config_path.push("src");
            config_path.push("resources");
            config_path.push("config.json");
            return Ok(config_path);
        }
    }
    Err("The source directory was not found in the ancestor path.".into())
}
pub fn get_config(path_buf: PathBuf) -> Configuration {
    let file =
        File::open(&path_buf).unwrap_or_else(|_| panic!("Failed to read file at: {path_buf:?}"));
    serde_json::from_reader(file).unwrap_or_else(|_| panic!("Failed to read config at ?????"))
}
