#[macro_export]
macro_rules! exec_child_process {
    ($path:expr, $args:expr) => {
        std::process::Command::new($path)
            .args($args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .stdin(std::process::Stdio::inherit())
            .spawn()
            .and_then(|process| process.wait_with_output())
    };
}
