use regex::Regex;
use serde::Deserialize;

use crate::{
    ops::{
        npm::NpmCommandLine,
        ops::{AddArgs, InstallArgs, PackageManagerOps},
        pnpm::PnpmCommandLine,
        yarn::YarnCommandLine,
        yarn_berry::YarnBerryCommandLine,
    },
    pm_metadata::PackageManagerMetadata,
};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum PackageManager {
    Npm(PackageManagerMetadata),
    Yarn(PackageManagerMetadata),
    YarnBerry(PackageManagerMetadata),
    Pnpm(PackageManagerMetadata),
}

impl From<PackageManagerMetadata> for PackageManager {
    fn from(metadata: PackageManagerMetadata) -> Self {
        match metadata.name.as_str() {
            "npm" => Self::Npm(metadata),
            "yarn" => Self::Yarn(metadata),
            "yarn@berry" => Self::YarnBerry(metadata),
            "pnpm" => Self::Pnpm(metadata),
            _ => unreachable!(),
        }
    }
}

impl PackageManager {
    fn execute<F, T>(&self, f: F) -> T
    where
        F: Fn(&dyn PackageManagerOps) -> T,
    {
        match self {
            Self::Npm(metadata) => f(&NpmCommandLine::new(metadata)),
            Self::Yarn(metadata) => f(&YarnCommandLine::new(metadata)),
            Self::YarnBerry(metadata) => f(&YarnBerryCommandLine::new(metadata)),
            Self::Pnpm(metadata) => f(&PnpmCommandLine::new(metadata)),
        }
    }

    pub fn install(&self, args: InstallArgs) -> anyhow::Result<Vec<String>> {
        self.execute(|pm| pm.install(args.clone()))
    }

    pub fn add(&self, args: AddArgs) -> anyhow::Result<Vec<String>> {
        self.execute(|pm| pm.add(args.clone()))
    }
}

impl PackageManager {
    fn metadata(&self) -> &PackageManagerMetadata {
        match self {
            Self::Npm(a) | Self::Yarn(a) | Self::YarnBerry(a) | Self::Pnpm(a) => a,
        }
    }

    pub fn name(&self) -> &str {
        self.metadata().name.as_str()
    }

    pub fn version(&self) -> &str {
        self.metadata().version.as_str()
    }

    pub fn hash_name(&self) -> Option<&str> {
        self.metadata().hash_name.as_deref()
    }

    pub fn hash_value(&self) -> Option<&str> {
        self.metadata().hash_value.as_deref()
    }

    pub fn parse(raw: &str) -> Option<Self> {
        let regex_str = r"^(?P<name>\w+)@(?P<version>[^+]+)(?:\+(?P<hash_method>sha\d*)\.(?P<hash_value>[a-fA-F0-9]+))?$";

        let regex = match Regex::new(regex_str) {
            Ok(regex) => regex,
            Err(_) => {
                eprintln!("Failed to create regex for package manager: {}", regex_str);
                return None;
            }
        };

        let captures = match regex.captures(raw) {
            Some(caps) => caps,
            None => {
                eprintln!("Failed to capture package manager: {}", raw);
                return None;
            }
        };

        let [name, version, hash_method, hash_value] =
            ["name", "version", "hash_method", "hash_value"]
                .map(|name| captures.name(name).map(|s| s.as_str().to_string()));

        let package_manager = match (name, version, hash_method, hash_value) {
            (Some(name), Some(version), None, None) => Self::from(PackageManagerMetadata {
                name,
                version,
                hash_name: None,
                hash_value: None,
            }),
            (Some(name), Some(version), Some(hash_method), Some(hash_value)) => {
                Self::from(PackageManagerMetadata {
                    name,
                    version,
                    hash_name: Some(hash_method),
                    hash_value: Some(hash_value),
                })
            }
            _ => {
                return None;
            }
        };

        Some(package_manager)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_manager_with_pnpm() {
        let pm = PackageManager::parse("pnpm@9.0.0").expect("Should parse PNPM package manager");

        assert!(matches!(pm, PackageManager::Pnpm(_)));

        let info = match pm {
            PackageManager::Pnpm(a) => a,
            _ => panic!("Expected Pnpm variant"),
        };

        assert_eq!(info.name, "pnpm");
        assert_eq!(info.version, "9.0.0");
    }

    #[test]
    fn test_parse_package_manager_with_pnpm_and_hash() {
        let pm = PackageManager::parse("pnpm@9.0.0+sha.1234567890");
        assert!(pm.is_some());

        if let Some(pm) = pm {
            assert_eq!(pm.name(), "pnpm");
            assert_eq!(pm.version(), "9.0.0");
            assert_eq!(pm.hash_name().as_deref(), Some("sha"));
            assert_eq!(pm.hash_value().as_deref(), Some("1234567890"));
        }
    }

    #[test]
    fn test_parse_package_manager_with_npm() {
        let pm = PackageManager::parse("npm@10.0.0");
        assert!(pm.is_some());

        if let Some(pm) = pm {
            assert_eq!(pm.name(), "npm");
            assert_eq!(pm.version(), "10.0.0");
        }
    }
}
