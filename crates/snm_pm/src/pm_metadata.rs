use crate::pm::PM;
use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManagerMetadata {
  pub version: String,
  pub hash: Option<PackageManagerHash>,
  pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManagerHash {
  pub method: String,
  pub value: String,
}

impl PackageManagerHash {
  pub fn new(method: String, value: String) -> Self {
    Self { method, value }
  }
}

impl Into<PM> for PackageManagerMetadata {
  fn into(self) -> PM {
    match self.name.as_str() {
      "npm" => PM::Npm(self),
      "yarn" => PM::Yarn(self),
      "pnpm" => PM::Pnpm(self),
      _ => unreachable!(),
    }
  }
}

impl PackageManagerMetadata {
  pub fn from_str(raw: &str) -> anyhow::Result<Self> {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
      Regex::new(r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$")
                .expect("Invalid regex pattern")
    });
    let captures = REGEX
      .captures(raw)
      .with_context(|| format!("Failed to capture package manager: {}", raw))?;

    let name = captures
      .name("name")
      .map(|m| m.as_str().to_string())
      .with_context(|| "Missing package name")?;

    let version = captures
      .name("version")
      .map(|m| m.as_str().to_string())
      .with_context(|| "Missing version")?;

    let hash = captures
      .name("hash_method")
      .and_then(|method| {
        captures
          .name("hash_value")
          .map(|value| (method.as_str().to_string(), value.as_str().to_string()))
      })
      .map(|(method, value)| PackageManagerHash::new(method, value));

    Ok(Self {
      version,
      hash,
      name,
    })
  }
}
