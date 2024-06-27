use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageManagerDownloadHash {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PackageManager {
    pub name: String,
    pub version: String,
    pub hash: Option<PackageManagerDownloadHash>,

    pub raw: String,
}
