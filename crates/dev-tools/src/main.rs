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

            let mut doc = toml_str.parse::<DocumentMut>().expect("msg");
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

        let mut major = original.clone();
        major.major = major.major + 1;

        let mut minor = original.clone();
        minor.minor = minor.minor + 1;

        let mut patch = original.clone();
        patch.patch = patch.patch + 1;

        // let x = patch.pre;

        let mut pre_version = original.clone();

        if let Some(pre) = pre_version.pre.split(".").next() {
            if let Ok(num) = pre.parse::<u64>() {
                pre_version.pre = Prerelease::new(format!("{}", num + 1).as_str()).unwrap();
            } else {
                eprintln!("无法解析 pre-release 版本号: {}", pre);
            }
        }

        a.push(major);
        a.push(minor);
        a.push(patch);
        a.push(pre_version);
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

        println!("{:?} version changed", path.display())
    });

    // let mut version = None;

    // for path in paths {
    //     let mut file = File::open(&path).expect("msg");
    //     let mut toml_str = String::new();
    //     file.read_to_string(&mut toml_str).expect("msg");

    //     let mut doc = toml_str.parse::<DocumentMut>().expect("msg");

    //     if version.is_none() {
    //         version = Some(doc["package"]["version"].as_str().unwrap().to_string());
    //     }

    //     println!(
    //         "{:?} --->",
    //         Version::parse(doc["package"]["version"].as_str().unwrap())
    //     );

    //     doc["package"]["version"] = value("0.0.1-85");

    //     let mut file = File::create(&path).expect("msg");
    //     file.write_all(doc.to_string().as_bytes()).expect("msg");

    //     file.flush().expect("msg");

    //     println!("{:?} version changed", path.display())
    // }

    Ok(())
}
