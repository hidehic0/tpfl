use crate::types;
use serde_yml;
use std::{fs, path::PathBuf};

pub fn load_config(path: PathBuf) -> Result<types::Config, std::string::String> {
    if path.is_dir() {
        return Err(format!("{} is dir", path.to_str().unwrap()));
    }
    if !path.is_file() {
        return Err(format!("Couldn't find {}", path.to_str().unwrap()));
    }

    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Err(format!("Couldn't load {}", path.to_str().unwrap())),
    };
    let res: types::Config = match serde_yml::from_str(&content) {
        Ok(c) => c,
        Err(_) => return Err(format!("Couldn't parse {}", path.to_str().unwrap())),
    };

    return Ok(res);
}
