use std::env::current_dir;

e2e::test1! {
  #[tokio::test]
  test_show_npm_version_when_missing_default_node_and_npm,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: [],
  |builder:e2e::CommandBuilder| => {
    builder.add_snapshot("npm -v")?;
    builder.assert_snapshots(|name,res| {
      insta::assert_snapshot!(name, res);
    })?;
  }
}

e2e::test1! {
  #[tokio::test]
  test_show_npm_version_when_default_npm_missing_but_node_exists,
  cwd: current_dir()?.join("tests").join("fixtures").join("empty"),
  envs: [],
  |builder:e2e::CommandBuilder| => {
    builder.add_snapshot("snm node install 20.0.0")?;
    builder.add_snapshot("snm node default 20.0.0")?;
    builder.add_snapshot("npm -v")?;
    builder.assert_snapshots(|name,res| {
      println!("name---->: {:?}", name);
      println!("res---->: {:?}", res);
      insta::assert_snapshot!(name, res);
    })?;
  }
}
