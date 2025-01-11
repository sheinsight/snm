use std::{
  env,
  path::{Path, PathBuf},
};

use config::{Config, File, FileFormat};

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";

const FILE_NAME: &str = ".npmrc";

const ETC_FILE_NAME: &str = "npmrc";

pub struct NpmrcReader {
  config: Option<Config>,
}

impl NpmrcReader {
  pub fn from<P: AsRef<Path>>(workspace: P) -> Self {
    let home_dir = match env::var_os("HOME") {
      Some(home_dir) => home_dir,
      None => return Self { config: None },
    };

    let prefix = env::var_os("PREFIX").unwrap_or_default();

    let sources = {
      #[cfg(target_os = "windows")]
      {
        // TODO: check if this is correct
        vec![
          env::var_os("APPDATA")
            .map(PathBuf::from)
            .map(|p| p.join(FILE_NAME))
            .unwrap_or_default(),
          PathBuf::from(&home_dir).join(FILE_NAME),
          workspace.as_ref().to_path_buf().join(FILE_NAME),
        ]
      }
      #[cfg(not(target_os = "windows"))]
      {
        vec![
          PathBuf::from("/")
            .join(&prefix)
            .join("etc")
            .join(ETC_FILE_NAME),
          PathBuf::from(&home_dir).join(FILE_NAME),
          workspace.as_ref().to_path_buf().join(FILE_NAME),
        ]
      }
    };

    let sources = sources
      .into_iter()
      .filter_map(|path| {
        path
          .exists()
          .then(|| File::from(path).format(FileFormat::Ini))
      })
      .collect::<Vec<_>>();

    Self {
      config: Config::builder().add_source(sources).build().ok(),
    }
  }

  pub fn read_registry_with_default(&self) -> String {
    let registry = self
      .config
      .as_ref()
      .and_then(|c| c.get_string("registry").ok())
      .unwrap_or(DEFAULT_REGISTRY.to_string());

    if registry.ends_with('/') {
      registry[..registry.len() - 1].to_string()
    } else {
      registry
    }
  }

  pub fn read(&self, key: &str) -> Option<String> {
    self.config.as_ref().and_then(|c| c.get_string(key).ok())
  }
}
