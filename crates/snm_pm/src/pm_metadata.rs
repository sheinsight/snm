use crate::pm::PM;
use anyhow::bail;
use lazy_regex::regex_captures;

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
    let Some((_, name, version, _hash_method, _hash_value)) = regex_captures!(
      r#"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$"#,
      raw
    ) else {
      bail!("Failed to capture package manager: {}", raw);
    };

    Ok(Self {
      version: version.to_string(),
      // TODO: 🤔 究竟以哪个 hash 为主？？ 远程的 meta 也有 hash 文件啊
      // 这是不是 corepack 的设计缺陷
      // hash: Some(PackageManagerHash::new(
      //   hash_method.to_string(),
      //   hash_value.to_string(),
      // )),
      hash: None,
      name: name.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use lazy_regex::regex_captures;

  #[test]
  fn test_parse_package_manager_metadata() {
    let (raw, name, version, hash_method, hash_value) = regex_captures!(r#"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$"#, "npm@10.0.0+sha.1234567890").unwrap();
    assert_eq!(raw, "npm@10.0.0+sha.1234567890");
    assert_eq!(name, "npm");
    assert_eq!(version, "10.0.0");
    assert_eq!(hash_method, "sha");
    assert_eq!(hash_value, "1234567890");
  }
}
