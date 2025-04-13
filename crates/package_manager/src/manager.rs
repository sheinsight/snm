use std::str::FromStr;

use anyhow::bail;
use lazy_regex::regex_captures;

use crate::PackageManagerKind;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManager {
  kind: PackageManagerKind,
  version: String,
}

impl PackageManager {
  pub fn new(kind: PackageManagerKind, version: String) -> Self {
    Self { kind, version }
  }

  pub fn name(&self) -> &str {
    self.kind.as_ref()
  }

  pub fn version(&self) -> &str {
    &self.version
  }
}

impl FromStr for PackageManager {
  type Err = anyhow::Error;

  fn from_str(raw: &str) -> Result<Self, Self::Err> {
    let Some((_, name, version)) = regex_captures!(
      r#"^(?P<name>npm|yarn|pnpm)@(?P<version>[^+]+)(?:\+.+)?$"#,
      raw
    ) else {
      bail!("Failed to capture package manager: {}", raw);
    };

    let kind = PackageManagerKind::try_from(name)
      .map_err(|_| anyhow::anyhow!("Unsupported package manager: {}, Raw: {}", name, raw))?;

    Ok(PackageManager::new(kind, version.to_string()))
  }
}
