use std::env::current_dir;

// use duct::cmd;
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

#[tokio::test]
async fn test_reqwest_download() -> Result<(), Box<dyn std::error::Error>> {
  let mock_server = e2e::http_mocker::HttpMocker::builder()?.build().await?;

  let node_url = format!(
    "{}{}",
    mock_server.uri(),
    "/v20.0.0/node-v20.0.0-win-x64.zip"
  );

  let out = current_dir()?.join("temp.zip");

  let res = snm_download_builder::DownloadBuilder::new()
    .retries(3)
    .timeout(30)
    .download(&node_url, &out)
    .await?;

  println!("test_reqwest_download ---->: {:?} {:?}", res, out.exists());

  //   let _url = "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json";
  //   let resp = if cfg!(target_os = "windows") {
  //     cmd!(
  //         "cmd",
  //         "/C",
  //         "certutil -urlcache -split -f https://raw.githubusercontent.com/nodejs/Release/main/schedule.json temp.json & type temp.json & del temp.json"
  //       )
  //       .stdout_capture()
  //       .stderr_capture()
  //       .read()?
  //   } else {
  //     cmd!(
  //       "curl",
  //       "-s",
  //       "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json"
  //     )
  //     .stdout_capture()
  //     .stderr_capture()
  //     .read()?
  //   };

  //   let response = reqwest::get(url).await?;
  //   println!("response---->: {:?}", response);
  //   assert!(response.status().is_success());
  //   let content = response.text().await?;
  //   assert!(content.contains("v0.8")); // 验证内容
  //   println!("content---->: {:?}", content);
  Ok(())
}
