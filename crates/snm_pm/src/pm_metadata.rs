use anyhow::bail;
use lazy_regex::regex_captures;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManagerMetadata {
  pub version: String,
  pub name: String,
}

// impl Into<PM> for PackageManagerMetadata {
//   fn into(self) -> PM {
//     match self.name.as_str() {
//       "npm" => PM::Npm(self),
//       "yarn" => PM::Yarn(self),
//       "pnpm" => PM::Pnpm(self),
//       _ => unreachable!(),
//     }
//   }
// }

impl PackageManagerMetadata {
  pub fn from_str(raw: &str) -> anyhow::Result<Self> {
    let Some((_, name, version)) = regex_captures!(
      r#"^(?P<name>npm|yarn|pnpm)@(?P<version>[^+]+)(?:\+.+)?$"#,
      raw
    ) else {
      bail!("Failed to capture package manager: {}", raw);
    };

    Ok(Self {
      version: version.to_string(),
      name: name.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use lazy_regex::regex_captures;

  #[test]
  fn test_parse_package_manager_metadata() {
    let (raw, name, version) = regex_captures!(
      r#"^(?P<name>npm|yarn|pnpm)@(?P<version>[^+]+)(?:\+.+)?$"#,
      "npm@10.0.0+sha.1234567890"
    )
    .unwrap();
    assert_eq!(raw, "npm@10.0.0+sha.1234567890");
    assert_eq!(name, "npm");
    assert_eq!(version, "10.0.0");
  }

  #[test]
  fn test_parse_package_manager_metadata_with_hash() {
    let (raw, name, version) = regex_captures!(
      r#"^(?P<name>npm|yarn|pnpm)@(?P<version>[^+]+)(?:\+.+)?$"#,
      "npm@10.0.0+sha.1234567890"
    )
    .unwrap();
    assert_eq!(raw, "npm@10.0.0+sha.1234567890");
    assert_eq!(name, "npm");
    assert_eq!(version, "10.0.0");
  }
}
