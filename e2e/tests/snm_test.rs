use std::env::current_dir;

use e2e::SnmEnv;

e2e::test1! {
    #[tokio::test]
    test_snm_install_node,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node list --compact")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_uninstall_node,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node list --compact")?;
        builder.add_snapshot("snm node uninstall 20.0.0")?;
        builder.add_snapshot("snm node list --compact")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_set_default_node,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("node -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_list,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node list")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("snm node list")?;
        builder.add_snapshot("snm node list --compact")?;
        builder.add_snapshot("snm node list --remote")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_list_with_strict_mode,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[SnmEnv::Strict("true".to_string())],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node list")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("snm node list")?;
        builder.add_snapshot("snm node list --compact")?;
        builder.add_snapshot("snm node list --remote")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_set_default_pnpm,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("pnpm -v")?;
        builder.add_snapshot("snm pnpm install 9.0.0")?;
        builder.add_snapshot("snm pnpm default 9.0.0")?;
        builder.add_snapshot("pnpm -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_set_default_npm_with_node_20,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.add_snapshot("snm npm install 9.0.0")?;
        builder.add_snapshot("snm npm default 9.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_set_default_yarn,
    cwd: current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("yarn -v")?;
        builder.add_snapshot("snm yarn install 1.22.22")?;
        builder.add_snapshot("snm yarn default 1.22.22")?;
        builder.add_snapshot("yarn -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_set_default_yarn4,
    cwd:current_dir()?.join("tests").join("fixtures").join("empty"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("yarn -v")?;
        builder.add_snapshot("snm yarn install 4.0.0")?;
        builder.add_snapshot("snm yarn default 4.0.0")?;
        builder.add_snapshot("yarn -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_with_node_20_npm,
    cwd: current_dir()?.join("tests").join("fixtures").join("snm_i_with_node_npm"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.exec("npm install")?;
        builder.add_snapshot("node index.cjs")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_snm_install_with_outside_npm,
    cwd:current_dir()?
    .join("tests")
    .join("fixtures")
    .join("test_snm_install_with_outside_pnpm"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.add_snapshot("snm node install 20.0.0")?;
        builder.add_snapshot("snm node default 20.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.add_snapshot("snm npm install 9.0.0")?;
        builder.add_snapshot("snm npm default 9.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.exec("npm install")?;
        builder.add_snapshot("node index.cjs")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}

e2e::test1! {
    #[tokio::test]
    test_when_node_modules_has_other_pm,
    cwd: current_dir()?.join("tests").join("fixtures").join("test_when_node_modules_has_other_pm"),

    envs:[],
    |builder:e2e::CommandBuilder| => {
        builder.exec("snm node install 20.0.0")?;
        builder.exec("snm node default 20.0.0")?;
        builder.exec("npm -v")?;
        builder.add_snapshot("snm npm install 9.0.0")?;
        builder.add_snapshot("snm npm default 9.0.0")?;
        builder.add_snapshot("npm -v")?;
        builder.assert_snapshots(|name,res| {
            insta::assert_snapshot!(name, res);
        })?;
    }
}
