#[macro_export]
macro_rules! exec_proxy_child_process {
    ($path:expr) => {{
        let args: Vec<String> = std::env::args().skip(1).collect();
        std::process::Command::new($path)
            .args(&args)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .stdin(std::process::Stdio::inherit())
            .spawn()
            .and_then(|process| process.wait_with_output())
    }};
}
