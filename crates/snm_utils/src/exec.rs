use std::{
    env::{self, current_exe},
    ffi::OsStr,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{bail, Error};

pub fn exec_cli<I, S>(dir: Vec<String>, bin_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let exe = current_exe()?;

    let exe_name = exe
        .file_name()
        .ok_or(Error::msg("exe file name not found"))?;

    let exe_dir = exe.parent().ok_or(Error::msg("exe parent dir not found"))?;

    let original_path = env::var("PATH")?;

    let new_path: String = format!("{}:{}", dir.join(":"), original_path);

    let has_binary = new_path
        .split(':') // 使用字符而不是字符串
        .filter(|path| !path.is_empty())
        .map(|path| Path::new(path).to_owned())
        .take_while(|path| {
            // println!(
            //     "path:{:?} exe_dir:{:?} bin_name:{:?} exe_name:{:?},dir:{:?}",
            //     path, exe_dir, bin_name, exe_name, dir
            // );

            return path != exe_dir && bin_name == exe_name;
        })
        .find(|path| {
            path.read_dir()
                .ok()
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path != &exe)
                .filter_map(|path| path.file_name().map(|n| n.to_owned()))
                .find(|name| name == bin_name)
                .is_some()
        });

    // let new_path = new_path
    //     .split(':') // 使用字符而不是字符串
    //     .filter(|path| !path.is_empty())
    //     .map(|path| Path::new(path).to_owned())
    //     .take_while(|path| {
    //         return path != exe_dir && bin_name == exe_name;
    //     })
    //     .map(|path| path.display().to_string())
    //     .collect::<Vec<String>>();

    if !has_binary.is_some() {
        bail!("command not found: {}", bin_name);
    }

    Command::new(&bin_name)
        .args(args)
        .env("PATH", new_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()?;

    Ok(())
}
