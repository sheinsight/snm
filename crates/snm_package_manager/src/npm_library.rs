use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NpmLibraryMeta {
    pub versions: HashMap<String, NpmLibraryVersionMeta>,
    pub time: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct NpmLibraryVersionMeta {
    pub name: String,
    pub version: String,
    pub license: Option<String>,
}
