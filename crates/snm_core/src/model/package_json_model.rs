use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJsonModel {
    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub bin: Option<Bin>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}
