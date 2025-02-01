use std::{
  env,
  path::{Path, PathBuf},
};

use config::{Config, File, FileFormat, FileSourceFile};

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";

const FILE_NAME: &str = ".npmrc";

#[allow(dead_code)]
const ETC_FILE_NAME: &str = "npmrc";

pub struct NpmrcReader {
  config: Option<Config>,
}

impl NpmrcReader {
  pub fn from<P: AsRef<Path>>(workspace: P) -> Self {
    let sources = Self::collect_config_sources(workspace.as_ref());

    let config = Config::builder().add_source(sources).build().ok();

    Self { config }
  }

  fn collect_config_sources(workspace: &Path) -> Vec<File<FileSourceFile, FileFormat>> {
    let mut sources = Vec::new();

    // 添加系统特定路径
    if let Some(sys_path) = Self::get_system_config_path() {
      sources.push(sys_path);
    }

    // 添加通用路径
    if let Some(home_dir) = dirs::home_dir() {
      sources.push(home_dir.join(FILE_NAME));
    }

    sources.push(workspace.join(FILE_NAME));

    sources
      .into_iter()
      .filter(|path| path.exists())
      .map(|path| File::from(path).format(FileFormat::Ini))
      .collect()
  }

  fn get_system_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
      env::var_os("APPDATA").map(|app_data| PathBuf::from(app_data).join(FILE_NAME))
    }
    #[cfg(not(target_os = "windows"))]
    {
      env::var_os("PREFIX").map(|prefix| {
        PathBuf::from("/")
          .join(prefix)
          .join("etc")
          .join(ETC_FILE_NAME)
      })
    }
  }

  pub fn read_registry_with_default(&self) -> String {
    self
      .config
      .as_ref()
      .and_then(|c| c.get_string("registry").ok())
      .unwrap_or_else(|| DEFAULT_REGISTRY.to_string())
      .trim_end_matches("/")
      .to_string()
  }

  pub fn read(&self, key: &str) -> Option<String> {
    self.config.as_ref().and_then(|c| c.get_string(key).ok())
  }
}
