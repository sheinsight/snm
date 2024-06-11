use std::env::{self, current_dir};

use snm_npmrc::parse_npmrc;

#[test]
fn test() {
    let c = current_dir()
        .unwrap()
        .join("tests")
        .join("features")
        .join("project");

    env::set_var(
        "PREFIX",
        current_dir()
            .unwrap()
            .join("tests")
            .join("features")
            .join("global")
            .display()
            .to_string(),
    );

    if let Some(config) = parse_npmrc(&c) {
        let registry = match config.get_string("registry") {
            Ok(registry) => registry,
            Err(_) => "error".to_string(),
        };
        assert_eq!(registry, "https://project.com".to_string());

        let cache = match config.get_string("cache") {
            Ok(registry) => registry,
            Err(_) => "error".to_string(),
        };

        assert_eq!(cache, "~/.hello".to_string());
    }
}
