#[cfg(test)]
mod snm_install_test {
    mod install_1 {
        use std::env::{self, current_dir, set_current_dir};

        use clap::Parser;
        use cli::{execute_cli, SnmCli};
        use std::fs;

        #[tokio::test]
        async fn test_pnpm_install() {
            env::set_var("SNM_NODE_INSTALL_STRATEGY", "auto");
            env::set_var("SNM_PACKAGE_MANAGER_INSTALL_STRATEGY", "auto");
            let c_dir = current_dir().expect("get current dir error");
            let test_dir = c_dir.join("tests").join("snm_install_pnpm");
            set_current_dir(&test_dir).expect("set current dir error");

            let cli = SnmCli::parse_from(["snm", "install"]);
            let lock = &test_dir.join("pnpm-lock.yaml");
            let _ = fs::remove_file(lock);

            assert!(!lock.exists());
            let res = execute_cli::execute_cli(cli).await;
            assert!(res.is_ok());
            assert!(lock.exists());
        }

        #[tokio::test]
        async fn test_npm_install() {
            env::set_var("SNM_NODE_INSTALL_STRATEGY", "auto");
            env::set_var("SNM_PACKAGE_MANAGER_INSTALL_STRATEGY", "auto");
            let c_dir = current_dir().expect("get current dir error");
            let test_dir = c_dir.join("tests").join("snm_install_npm");
            set_current_dir(&test_dir).expect("set current dir error");

            let cli = SnmCli::parse_from(["snm", "install"]);
            let lock = &test_dir.join("package-lock.json");
            let _ = fs::remove_file(lock);

            assert!(!lock.exists());
            let res = execute_cli::execute_cli(cli).await;
            assert!(res.is_ok());
            assert!(lock.exists());
        }
    }
}
