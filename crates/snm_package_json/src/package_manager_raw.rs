use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct PackageJsonRaw {
    pub name: Option<String>,

    pub version: Option<String>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub bin: Option<Bin>,
}
