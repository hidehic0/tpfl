use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
}

#[derive(Deserialize, PartialEq)]
pub struct Template {
    pub name: String,
    #[serde(rename(deserialize = "filename"))]
    pub file_name: String,
    #[serde(rename(deserialize = "type"))]
    pub file_type: String,
    pub path: String,
}
