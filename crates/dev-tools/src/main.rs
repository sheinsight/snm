use std::{
  fs::File,
  io::{Read as _, Write as _},
  path::PathBuf,
};

use glob::glob;
use inquire::Select;
use semver::{Prerelease, Version};
use toml_edit::{value, DocumentMut};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let paths = glob("crates/**/Cargo.toml")
    .expect("Failed to read glob pattern")
    .into_iter()
    .filter_map(|item| item.ok())
    .collect::<Vec<PathBuf>>();

  let x = paths
    .iter()
    .map(|path| {
      let mut file = File::open(&path).expect("msg");
      let mut toml_str = String::new();
      file.read_to_string(&mut toml_str).expect("msg");

      let doc = toml_str.parse::<DocumentMut>().expect("msg");
      doc["package"]["version"]
        .clone()
        .as_str()
        .unwrap()
        .to_string()
    })
    .collect::<Vec<_>>();

  let mut a: Vec<Version> = vec![];

  if let Some(first) = x.first() {
    let original = Version::parse(first)?;

    let major = Version::new(original.major + 1, 0, 0);
    let minor = Version::new(original.major, original.minor + 1, 0);
    let patch = Version::new(original.major, original.minor, original.patch + 1);

    let mut pre_major = original.clone();
    pre_major.major = pre_major.major + 1;
    pre_major.minor = 0;
    pre_major.patch = 0;
    pre_major.pre = Prerelease::new("0").unwrap();

    let mut pre_minor = original.clone();
    pre_minor.major = pre_minor.major;
    pre_minor.minor = pre_minor.minor + 1;
    pre_minor.patch = 0;
    pre_minor.pre = Prerelease::new("0").unwrap();

    // let mut pre_patch = original.clone();
    // pre_patch.major = pre_patch.major;
    // pre_patch.minor = pre_patch.minor;
    // pre_patch.patch = pre_patch.patch + 1;
    // pre_patch.pre = Prerelease::new("0").unwrap();

    let mut pre_patch = original.clone();
    pre_patch.patch = pre_patch.patch + 1;
    pre_patch.pre = Prerelease::new("0").unwrap();

    let mut pre_release = original.clone();
    pre_release.pre = Prerelease::new("0").unwrap();

    if let Some(pre) = pre_release.pre.split(".").next() {
      if let Ok(num) = pre.parse::<u64>() {
        pre_release.pre = Prerelease::new(format!("{}", num + 1).as_str()).unwrap();
      } else {
        eprintln!("无法解析 pre-release 版本号: {}", pre);
      }
    }

    a.push(major);
    a.push(minor);
    a.push(patch);
    a.push(pre_major);
    a.push(pre_minor);
    a.push(pre_patch);
    a.push(pre_release);
  }
  println!("{:?}", a);

  let ans = Select::new(
    "What's your favorite fruit?",
    a.into_iter()
      .map(|item| item.to_string())
      .collect::<Vec<_>>(),
  )
  .prompt()?;

  paths.iter().for_each(|path| {
    let mut file = File::open(&path).expect("msg");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).expect("msg");

    let mut doc = toml_str.parse::<DocumentMut>().expect("msg");

    doc["package"]["version"] = value(ans.clone());

    let mut file = File::create(&path).expect("msg");
    file.write_all(doc.to_string().as_bytes()).expect("msg");

    file.flush().expect("msg");

    println!("{:?} version changed", &path.display());
  });

  run_git_command(&["add", "."]);
  run_git_command(&["commit", "-m", &format!("chore: bump version to {}", ans)]);
  run_git_command(&["tag", &format!("v{}", ans)]);
  // run_git_command(&["push"]);

  println!("Please run `git push --tags` to push the tags to the remote repository.");

  Ok(())
}

fn run_git_command(args: &[&str]) {
  let output = std::process::Command::new("git")
    .args(args)
    .output()
    .expect("Failed to execute git command");

  if !output.status.success() {
    panic!(
      "Command executed with failing error code: {:?}",
      output.status.code()
    );
  }
}
