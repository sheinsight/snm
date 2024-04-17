use std::{env::current_dir, fs};

use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use semver::{Prerelease, Version};
use snm_core::model::{PackageJson, SnmError};

pub fn bump_impl() -> Result<(), SnmError> {
    let package_json = PackageJson::from_here()?;
    let current_version =
        Version::parse(package_json.version.unwrap_or("0.0.0".to_string()).as_str())
            .expect("parse version error");
    let prerelease_number = current_version.pre.parse::<u8>().unwrap_or(0) + 1;

    let major = current_version.major;
    let minor = current_version.minor;
    let patch = current_version.patch;

    let zero_pre_release = Prerelease::new("0").expect("create prerelease error 0");

    let versions_and_strings = vec![
        create_version_and_string("major", major + 1, 0, 0, None)?,
        create_version_and_string("minor", major, minor + 1, 0, None)?,
        create_version_and_string("patch", major, minor, patch + 1, None)?,
        create_version_and_string("premajor", major + 1, 0, 0, Some(zero_pre_release.clone()))?,
        create_version_and_string(
            "preminor",
            major,
            minor + 1,
            0,
            Some(zero_pre_release.clone()),
        )?,
        create_version_and_string("prepatch", major, minor, patch + 1, Some(zero_pre_release))?,
        create_version_and_string(
            "prerelease",
            major,
            minor,
            patch,
            Some(
                Prerelease::new(prerelease_number.to_string().as_str())
                    .expect(format!("create prerelease error {}", prerelease_number).as_str()),
            ),
        )?,
    ];

    let selections: Vec<String> = versions_and_strings
        .iter()
        .map(|(_, s)| s.clone())
        .collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "请选择要升级的版本号: {} ",
            current_version.to_string().bright_purple()
        ))
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("bump_impl Select error");

    let dir = current_dir().expect("get current dir failed");

    let c = fs::read_to_string(dir.join("package.json")).expect(
        format!(
            "bump_impl read_to_string error {:?}",
            dir.join("package.json").display()
        )
        .as_str(),
    );

    let version_regex = Regex::new(r#""version"\s*:\s*"[^"]*""#).expect("create regex error");
    let replacement = format!(
        r#""version": "{}""#,
        versions_and_strings[selection].0.to_string()
    );

    let x = version_regex.replace(&c, replacement.as_str());

    fs::write(dir.join("package.json"), x.to_string()).expect(
        format!(
            "bump_impl write error {:?}",
            dir.join("package.json").display()
        )
        .as_str(),
    );

    println!(
        "您选择了: {} , {:?}",
        selections[selection], versions_and_strings[selection].0
    );

    Ok(())
}

fn create_version_and_string(
    version_type: &str,
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<Prerelease>,
) -> Result<(Version, String), SnmError> {
    let mut new_version = Version::new(major, minor, patch);
    if let Some(p) = pre {
        new_version.pre = p.clone();
    }
    let version_string = format!(
        "{:<12} {}",
        version_type,
        new_version.to_string().bright_black()
    );
    Ok((new_version, version_string))
}
