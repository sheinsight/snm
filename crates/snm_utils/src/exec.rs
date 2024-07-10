use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

use crate::snm_error::SnmError;

pub fn exec_cli<T: AsRef<OsStr>, I, S>(bin_name: T, args: I) -> Result<(), SnmError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(bin_name)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .and_then(|process| process.wait_with_output())?;

    if !output.status.success() {
        return Err(SnmError::SNMBinaryProxyFail {
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    print!("{}", String::from_utf8_lossy(&output.stdout).to_string());

    Ok(())
}
