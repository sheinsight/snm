use serde::{Deserialize, Deserializer, Serialize};
use std::{borrow::Cow, fmt};

#[derive(Deserialize, Debug, Serialize)]
pub struct NodeModel {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
    pub lts: Lts,
    pub security: bool,
    pub end: Option<String>,
    pub current: Option<String>,
    pub deprecated: Option<bool>,
}

impl fmt::Display for NodeModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "version: {:<10}, lts: {:<10}, date: {:<12}, end: {:<12}, npm: {:<14}, v8: {:<14}, uv: {:<12}, zlib: {:<18}, openssl: {:<14}, modules: {:<12}, deprecated: {}",
            self.version,
            match &self.lts {
                Lts::Str(s) => Cow::Borrowed(s),
                Lts::Bool(b) => Cow::Owned(b.to_string()),
            },
            self.date,
            self.end.as_deref().unwrap_or("None"),
            self.npm.as_deref().unwrap_or("None"),
            self.v8,
            self.uv.as_deref().unwrap_or("None"),
            self.zlib.as_deref().unwrap_or("None"),
            self.openssl.as_deref().unwrap_or("None"),
            self.modules.as_deref().unwrap_or("None"),
            self.deprecated.map_or("None".to_string(), |v| v.to_string())
        )
    }
}

#[derive(Debug)]
pub enum Lts {
    Str(String),
    Bool(bool),
}

impl Serialize for Lts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            Lts::Str(s) => serializer.serialize_str(s),
            Lts::Bool(b) => serializer.serialize_bool(*b),
        }
    }
}

// 自定义反序列化
impl<'de> Deserialize<'de> for Lts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(s) => Ok(Lts::Str(s)),
            serde_json::Value::Bool(b) => Ok(Lts::Bool(b)),
            _ => Err(serde::de::Error::custom("expected a string or a bool")),
        }
    }
}
