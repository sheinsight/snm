use std::{env, ops::Deref, path::PathBuf};

use config::{Config, File, FileFormat};

pub fn parse_npmrc(workspace: &PathBuf) -> Option<Config> {
    let home_dir = match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(_) => {
            return None;
        }
    };

    let prefix = match env::var("PREFIX") {
        Ok(prefix) => prefix,
        Err(_) => "".to_string(),
    };

    let builder = vec![
        PathBuf::from("/").join(prefix).join("etc").join("npmrc"),
        PathBuf::from(home_dir).join(".npmrc"),
        PathBuf::from(workspace).join(".npmrc"),
    ]
    .iter()
    .filter(|item| item.exists())
    .fold(Config::builder(), |build, item| {
        build.add_source(File::from(item.deref()).format(FileFormat::Ini))
    });

    if let Ok(config) = builder.build() {
        Some(config)
    } else {
        None
    }
}
