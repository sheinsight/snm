use std::{env, ffi::OsString, path::PathBuf};

use config::{Config, File, FileFormat};

const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org/";

pub struct Npmrc {
    config: Option<Config>,
}

impl Npmrc {
    pub fn from(workspace: &PathBuf) -> Self {
        let home_dir = match env::var_os("HOME") {
            Some(home_dir) => home_dir,
            None => return Self { config: None },
        };

        let prefix = match env::var_os("PREFIX") {
            Some(prefix) => prefix,
            None => OsString::from(""),
        };

        let sources = vec![
            PathBuf::from("/").join(prefix).join("etc").join("npmrc"),
            PathBuf::from(home_dir).join(".npmrc"),
            PathBuf::from(workspace).join(".npmrc"),
        ]
        .into_iter()
        .filter_map(|path| {
            path.exists()
                .then(|| File::from(path).format(FileFormat::Ini))
        })
        .collect::<Vec<_>>();

        Self {
            config: Config::builder().add_source(sources).build().ok(),
        }
    }

    pub fn read_registry_with_default(&self) -> String {
        self.config
            .as_ref()
            .and_then(|c| c.get_string("registry").ok())
            .unwrap_or(DEFAULT_REGISTRY.to_string())
    }

    pub fn read(&self, key: &str) -> Option<String> {
        self.config.as_ref().and_then(|c| c.get_string(key).ok())
    }
}
