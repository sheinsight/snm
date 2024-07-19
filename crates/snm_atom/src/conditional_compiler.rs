pub fn get_tarball_ext() -> String {
    match std::env::consts::OS {
        "windows" => "zip".to_string(),
        "linux" => "tar.xz".to_string(),
        "macos" => "tar.xz".to_string(),
        _ => "unknown".to_string(), // 默认情况
    }
}

pub fn get_arch() -> String {
    match std::env::consts::ARCH {
        "x86" => "x86".to_string(),
        "x86_64" => "x64".to_string(),
        "arm" => "armv7l".to_string(),
        "aarch64" => "arm64".to_string(),
        "powerpc64" => "ppc64".to_string(),
        "powerpc64le" => "ppc64le".to_string(),
        "s390x" => "s390x".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn get_os() -> String {
    match std::env::consts::OS {
        "macos" => "darwin".to_string(),
        "windows" => "win".to_string(),
        _ => std::env::consts::OS.to_string(),
    }
}
