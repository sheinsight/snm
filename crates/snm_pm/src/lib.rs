use colored::Colorize;
use npm::Npm;
use snm_core::{
    model::{PackageJson, SnmError},
    println_success,
};
use std::{
    io::{stdout, Write as _},
    path::PathBuf,
};
use yarn::Yarn;
pub mod npm;
mod path;
pub mod utils;
pub mod yarn;

pub async fn get_manager_bin_file_path(expect_package_manager: &str) -> Result<PathBuf, SnmError> {
    let pkg = PackageJson::from_file_path(None)?;

    let version_parsed = pkg.parse_package_manager()?;

    if version_parsed.package_manager != expect_package_manager {
        return Err(SnmError::NotMatchPackageManager {
            expect: expect_package_manager.to_string(),
            actual: version_parsed.package_manager,
        });
    }

    let msg = format!(
        "Use PackageManager {}@{}",
        version_parsed.package_manager.bright_green(),
        version_parsed.version.bright_green()
    );

    let bin_path = match expect_package_manager {
        "yarn" => {
            let yarn = Yarn::new(version_parsed)?;
            yarn.get_bin_path("yarn").await?
        }
        "npm" => {
            let npm = Npm::new(version_parsed);
            npm.get_bin_path("npm").await?
        }
        "pnpm" => {
            let npm = Npm::new(version_parsed);
            npm.get_bin_path("pnpm").await?
        }
        _ => todo!(),
    };

    let mut stdout = stdout();

    println_success!(stdout, "{}", msg);

    std::io::stdout().flush()?;

    Ok(bin_path)
}
