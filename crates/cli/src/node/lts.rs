use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
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
