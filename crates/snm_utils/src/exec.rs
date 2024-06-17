use std::{
    ffi::OsStr,
    process::{Command, Stdio},
};

pub fn exec_cli<T: AsRef<OsStr>, I, S>(bin_name: T, args: I)
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
        .and_then(|process| process.wait_with_output());

    if let Err(_) = output {
        panic!("snm proxy execute failed");
    }
}
