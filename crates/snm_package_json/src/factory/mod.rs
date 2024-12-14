use std::{fs, ops::Not, process::exit};

use clap::Subcommand;
use colored::Colorize;
use dialoguer::Confirm;

use crate::{downloader::PackageManagerDownloader, pm_metadata::PackageManagerMetadata};

#[derive(Debug, clap::Args)]
pub struct FactoryCommandArgs {
    #[arg(help = "Package manager version")]
    pub version: String,
}

#[derive(Subcommand, Debug)]
pub enum PackageManagerFactoryCommands {
    Install(FactoryCommandArgs),
    Default(FactoryCommandArgs),
    Uninstall(FactoryCommandArgs),
}

pub struct PackageManagerFactory<'a> {
    metadata: &'a PackageManagerMetadata<'a>,
}

impl<'a> PackageManagerFactory<'a> {
    pub fn new(metadata: &'a PackageManagerMetadata<'a>) -> Self {
        Self { metadata }
    }

    pub async fn set_default(&self) -> anyhow::Result<()> {
        let metadata = self.metadata;

        let dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join(&metadata.version);

        let file = dir.join("package.json");

        if file.try_exists()?.not() {
            let confirmed = Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is not installed, do you want to install it ?",
                    &metadata.version
                ))
                .interact()?;
            if confirmed {
                self.install().await?;
            }
        }

        let default_dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join("default");

        if default_dir.try_exists()? {
            fs::remove_dir_all(&default_dir)?;
        }

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&dir, &default_dir)?;
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(&dir, &default_dir)?;
        }

        println!(
            "🎉 {} v{} is now default",
            &metadata.library_name, &metadata.version
        );

        Ok(())
    }

    pub async fn install(&self) -> anyhow::Result<()> {
        let metadata = self.metadata;

        let dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join(&metadata.version);

        let file = dir.join("package.json");

        if file.try_exists()? {
            let confirm = Confirm::new()
                .with_prompt(format!(
                    "🤔 v{} is already installed, do you want to reinstall it ?",
                    &metadata.version
                ))
                .interact()?;

            if confirm {
                fs::remove_dir_all(&dir)?;
            } else {
                exit(1);
            }
        }

        PackageManagerDownloader::new(self.metadata)
            .download_pm(&metadata.version)
            .await?;

        Ok(())
    }

    pub async fn uninstall(&self) -> anyhow::Result<()> {
        let metadata = self.metadata;

        let dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join(&metadata.version);

        if dir.try_exists()?.not() {
            let msg = format!(
                "🤔 {} v{} is not installed",
                &metadata.library_name, &metadata.version
            );
            eprintln!("{}", msg.bright_red());
            return Ok(());
        }

        let default_dir = metadata
            .config
            .node_modules_dir
            .join(&metadata.library_name)
            .join("default");

        if default_dir.try_exists()? {
            if default_dir.read_link()?.eq(&dir) {
                fs::remove_dir_all(&default_dir)?;
                println!(
                    "🎉 Node v{} is uninstalled , Now there is no default node .",
                    &metadata.version.bright_green()
                );
            }
        }

        fs::remove_dir_all(&dir)?;

        println!("🎉 v{} is uninstalled", &metadata.version);

        Ok(())
    }

    pub async fn list(&self) -> anyhow::Result<()> {
        todo!()
    }
}
