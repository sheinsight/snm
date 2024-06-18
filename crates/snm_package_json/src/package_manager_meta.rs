use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageManagerDownloadHash {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PackageManager {
    pub name: String,
    pub version: String,
    pub hash: Option<PackageManagerDownloadHash>,

    pub raw: String,
}
