use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Bin {
    Str(String),
    Map(HashMap<String, String>),
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageJsonRaw {
    pub name: Option<String>,

    pub version: Option<String>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,

    pub bin: Option<Bin>,
}
