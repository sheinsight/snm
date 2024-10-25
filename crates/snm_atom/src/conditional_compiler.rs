pub const fn get_tarball_ext() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "zip"
    }
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        "tar.xz"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "unknown"
    }
}

pub const fn get_arch() -> &'static str {
    #[cfg(target_arch = "x86")]
    {
        "x86"
    }
    #[cfg(target_arch = "x86_64")]
    {
        "x64"
    }
    #[cfg(target_arch = "arm")]
    {
        "armv7l"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "arm64"
    }
    #[cfg(target_arch = "powerpc64")]
    {
        "ppc64"
    }
    #[cfg(target_arch = "powerpc64le")]
    {
        "ppc64le"
    }
    #[cfg(target_arch = "s390x")]
    {
        "s390x"
    }
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "powerpc64le",
        target_arch = "s390x"
    )))]
    {
        "unknown"
    }
}

pub const fn get_os() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "darwin"
    }
    #[cfg(target_os = "windows")]
    {
        "win"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        "unknown"
    }
}
