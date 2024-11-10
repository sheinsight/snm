use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageManagerMetadata {
    pub name: String,
    pub version: String,
    pub hash_name: Option<String>,
    pub hash_value: Option<String>,
}
