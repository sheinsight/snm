use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
pub struct NpmConfig {
    pub registry: Option<String>,
}

impl NpmConfig {
    pub fn new() -> Self {
        Self { registry: None }
    }

    pub fn load_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        for line in io::BufReader::new(file).lines() {
            let line = line?;
            if let Some((key, value)) = line.split_once('=') {
                if key.trim() == "registry" {
                    self.registry = Some(value.trim().to_string());
                }
            }
        }
        Ok(())
    }

    pub fn load(&mut self, workspace: &Path) -> io::Result<()> {
        // Load system-level .npmrc (/etc/npmrc)
        if cfg!(unix) {
            let system_path = Path::new("/etc/npmrc");
            if system_path.exists() {
                self.load_from_file(system_path)?;
            }
        }

        // Load global .npmrc (prefix/etc/npmrc)
        if let Ok(prefix) = std::env::var("NPM_CONFIG_PREFIX") {
            let global_path = Path::new(&prefix).join("etc/npmrc");
            if global_path.exists() {
                self.load_from_file(&global_path)?;
            }
        }

        // Load user .npmrc (~/.npmrc)
        if let Some(home_dir) = dirs::home_dir() {
            let user_path = home_dir.join(".npmrc");
            if user_path.exists() {
                self.load_from_file(&user_path)?;
            }
        }

        // Load project .npmrc (./.npmrc)
        let project_path = workspace.join(".npmrc");
        if project_path.exists() {
            self.load_from_file(&project_path)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_load_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".npmrc");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "registry=https://example.com").unwrap();

        let mut config = NpmConfig::new();
        config.load_from_file(&file_path).unwrap();

        assert_eq!(config.registry, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_load_multiple_files() {
        let dir = tempdir().unwrap();
        let system_path = dir.path().join("etc/npmrc");
        let global_path = dir.path().join("global/npmrc");
        let user_path = dir.path().join("user/npmrc");
        let project_path = dir.path().join(".npmrc");

        // Create fake directories
        std::fs::create_dir_all(system_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(global_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(user_path.parent().unwrap()).unwrap();

        // Write different registry settings to each file
        let mut file = File::create(&system_path).unwrap();
        writeln!(file, "registry=https://system.com").unwrap();

        let mut file = File::create(&global_path).unwrap();
        writeln!(file, "registry=https://global.com").unwrap();

        let mut file = File::create(&user_path).unwrap();
        writeln!(file, "registry=https://user.com").unwrap();

        let mut file = File::create(&project_path).unwrap();
        writeln!(file, "registry=https://project.com").unwrap();

        let mut config = NpmConfig::new();

        // Manually load each file in the correct order
        if system_path.exists() {
            config.load_from_file(&system_path).unwrap();
        }
        if global_path.exists() {
            config.load_from_file(&global_path).unwrap();
        }
        if user_path.exists() {
            config.load_from_file(&user_path).unwrap();
        }
        if project_path.exists() {
            config.load_from_file(&project_path).unwrap();
        }

        // Ensure the project path value is used as it should be the last loaded
        assert_eq!(config.registry, Some("https://project.com".to_string()));
    }

    #[test]
    fn test_load() {
        let dir = tempdir().unwrap();
        let project_path = dir.path();

        let mut file = File::create(project_path.join(".npmrc")).unwrap();
        writeln!(file, "registry=https://example.com").unwrap();

        let mut config = NpmConfig::new();
        config.load(&project_path).unwrap();

        assert_eq!(config.registry, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_load_priority() {
        let dir = tempdir().unwrap();
        let system_path = dir.path().join("etc/npmrc");
        let global_path = dir.path().join("global/npmrc");
        let user_path = dir.path().join("user/npmrc");
        let project_path = dir.path().join(".npmrc");

        // Create fake directories
        std::fs::create_dir_all(system_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(global_path.parent().unwrap()).unwrap();
        std::fs::create_dir_all(user_path.parent().unwrap()).unwrap();

        // Write different registry settings to each file
        let mut file = File::create(&system_path).unwrap();
        writeln!(file, "registry=https://system.com").unwrap();

        let mut file = File::create(&global_path).unwrap();
        writeln!(file, "registry=https://global.com").unwrap();

        let mut file = File::create(&user_path).unwrap();
        writeln!(file, "registry=https://user.com").unwrap();

        // let mut file = File::create(&project_path).unwrap();
        // writeln!(file, "registry=https://project.com").unwrap();

        let mut config = NpmConfig::new();

        // Set the environment variable for the global path
        std::env::set_var("NPM_CONFIG_PREFIX", global_path.parent().unwrap());

        // Set the home directory for the user path
        std::env::set_var("HOME", user_path.parent().unwrap());

        // Load the configuration
        config.load(dir.path()).unwrap();

        // Ensure the project path value is used as it should be the last loaded
        assert_eq!(config.registry, Some("https://user.com".to_string()));
    }
}
