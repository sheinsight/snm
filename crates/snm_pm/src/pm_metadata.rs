use std::env;

use anyhow::Context;
use once_cell::sync::Lazy;
use regex::Regex;
use snm_config::SnmConfig;
use snm_utils::{
  consts::{ENV_KEY_FOR_SNM_PM, YARNPKG_PACKAGE_NAME, YARN_PACKAGE_NAME},
  ver::ver_gt_1,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageManagerMetadata<'a> {
  pub library_name: String,
  pub version: String,
  // pub hash_name: Option<String>,
  // pub hash_value: Option<String>,
  pub hash: Option<PackageManagerHash>,
  pub config: &'a SnmConfig,
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

impl<'a> PackageManagerMetadata<'a> {
  pub fn from_str(raw: &str, config: &'a SnmConfig) -> anyhow::Result<Self> {
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

    let library_name = if name != YARN_PACKAGE_NAME {
      &name
    } else {
      if ver_gt_1(&version)? {
        YARNPKG_PACKAGE_NAME
      } else {
        YARN_PACKAGE_NAME
      }
    };

    env::set_var(ENV_KEY_FOR_SNM_PM, raw);

    Ok(Self {
      library_name: library_name.to_owned(),
      version,
      hash,
      config,
      name,
    })
  }
}
