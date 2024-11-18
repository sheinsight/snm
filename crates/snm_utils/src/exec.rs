use std::{
    env::{self, current_exe},
    ffi::OsStr,
    path::Path,
    process::{Command, Stdio},
};

use crate::snm_error::SnmError;

pub fn exec_cli<T: AsRef<OsStr>, I, S>(
    dir: Vec<String>,
    bin_name: T,
    args: I,
) -> Result<(), SnmError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let exe = current_exe()?;

    let original_path = env::var("PATH")?;

    let new_path: String = format!("{}:{}", dir.join(":"), original_path);

    let has_binary = new_path
        .split(':') // 使用字符而不是字符串
        .filter(|path| !path.is_empty())
        .find(|path| {
            Path::new(path)
                .read_dir()
                .ok()
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path != &exe)
                .filter_map(|path| path.file_name().map(|n| n.to_owned()))
                .find(|name| name == bin_name.as_ref())
                .is_some()
        })
        .is_some();

    if !has_binary {
        return Err(SnmError::SNMBinaryProxyFail {
            stderr: format!(
                "{} not found in path",
                bin_name.as_ref().to_owned().to_string_lossy()
            ),
        });
    }

    let status = Command::new(&bin_name)
        .args(args)
        .env("PATH", new_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()?;

    if !status.success() {
        return Err(SnmError::SNMBinaryProxyFail {
            stderr: "Command execution failed".to_string(),
        });
    }

    Ok(())
}
