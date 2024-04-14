#[cfg(target_os = "windows")]
pub fn get_tarball_ext() -> String {
    "zip".to_string()
}

#[cfg(target_os = "linux")]
pub fn get_tarball_ext() -> String {
    "tar.xz".to_string()
}

#[cfg(target_os = "macos")]
pub fn get_tarball_ext() -> String {
    "tar.xz".to_string()
}

#[cfg(target_arch = "x86")]
pub fn get_arch() -> String {
    "x86".to_string()
}

#[cfg(target_arch = "x86_64")]
pub fn get_arch() -> String {
    "x64".to_string()
}

#[cfg(target_arch = "arm")]
pub fn get_arch() -> String {
    "armv7l".to_string()
}

#[cfg(target_arch = "aarch64")]
pub fn get_arch() -> String {
    "arm64".to_string()
}

#[cfg(target_arch = "ppc64")]
pub fn get_arch() -> String {
    "ppc64".to_string()
}

#[cfg(target_arch = "ppc64le")]
pub fn get_arch() -> String {
    "ppc64le".to_string()
}

#[cfg(target_arch = "s390x")]
pub fn get_arch() -> String {
    "s390x".to_string()
}

#[cfg(target_os = "macos")]
pub fn get_os() -> String {
    "darwin".to_string()
}

#[cfg(not(target_os = "macos"))]
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}
