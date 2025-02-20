use std::env::current_exe;
use std::process::Command;
use std::{fs, ops::Not};

use clap::CommandFactory;
use colored::*;
use snm_config::snm_config::SnmConfig;
use snm_pm::pm::SPM;
use snm_utils::exec::exec_cli;
use tracing::trace;

use crate::cli::SnmCli;
use crate::manage_command::NodeManageCommands;
use crate::snm_command::SnmCommands;

pub async fn execute_cli(cli: SnmCli, snm_config: SnmConfig) -> anyhow::Result<()> {
  trace!(
    r#"Start execute cli 
{}"#,
    cli
  );

  match cli.command {
    SnmCommands::Node { command } => {
      let nm = snm_node::factory::NodeFactory::new(&snm_config);

      match command {
        NodeManageCommands::Default(args) => {
          nm.set_default(args).await?;
        }
        NodeManageCommands::Install(args) => {
          nm.install(args).await?;
        }
        NodeManageCommands::Uninstall(args) => {
          nm.uninstall(args).await?;
        }
        NodeManageCommands::List(args) => {
          nm.list(args).await?;
        }
      }
    }
    SnmCommands::Install(_) | SnmCommands::Uninstall(_) | SnmCommands::Run(_) => {
      let spm = SPM::try_from(&snm_config.workspace, &snm_config)?;

      let pm = spm.pm;

      let handler = pm.get_ops();
      let commands = match cli.command {
        SnmCommands::Install(install_args) => handler.install(install_args),
        SnmCommands::Uninstall(remove_args) => handler.remove(remove_args),
        SnmCommands::Run(run_args) => handler.run(run_args),
        _ => unreachable!(),
      }?;

      exec_cli(&commands, &vec![], false)?;
    }
    SnmCommands::SetUp => {
      setup_fig()?;
      setup_symlink()?;
    }
    SnmCommands::AiCommit => {
      let unified_diff = Command::new("git")
        .args(["diff", "--staged", "--unified=3"])
        .output()?;

      let stat_diff = Command::new("git")
        .args(["diff", "--staged", "--stat"])
        .output()?;

      let unified_content = String::from_utf8_lossy(&unified_diff.stdout);
      let stat_content = String::from_utf8_lossy(&stat_diff.stdout);

      // println!("Unified diff:\n{}", unified_content);
      // println!("Stat diff:\n\n{}", stat_content);

      let client = async_openai::Client::new();

      let request = async_openai::types::CreateChatCompletionRequestArgs::default()
        // .max_tokens(512u32)
        .model("gpt-4o")
        .messages([
          async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
            .content("你是一个资深程序员，擅长各种编程语言，特别是 javascript 、typescript、rust、java，并且你还精通 git 。")
            .build()?
            .into(),
          async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(format!(r#"
请针对我的 git diff 内容生成一份标准的 中文 commit msg 信息,满足以下条件
  - 尽可能的推断出代码改动的意图
  - 请不要回复多余的信息，直接回复 commit msg 的内容
  - commit msg 遵循 angular commit message 格式
"#))
            .build()?
            .into(),
          async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(format!(r#"你可以从接下来的内容中获取 哪些文件被修改、每个文件增删了多少行"#))
            .build()?
            .into(),
            async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(stat_content.to_string())
            .build()?
            .into(),
            async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(format!(r#"你可以从接下来的内容中获取具体每一个文件的修改内容"#))
            .build()?
            .into(),
          async_openai::types::ChatCompletionRequestUserMessageArgs::default()
            .content(unified_content.to_string())
            .build()?
            .into(),
        ])
        .build()?;

      // println!("{}", serde_json::to_string(&request).unwrap());

      let response = client.chat().create(request).await?;

      // println!("\nResponse:\n");
      for choice in response.choices {
        // println!(
        //   "{}: Role: {}",
        //   choice.index, choice.message.role
        // );
        println!("{}", choice.message.content.unwrap());
      }
    }
  }

  Ok(())
}

fn setup_fig() -> anyhow::Result<()> {
  let mut output = Vec::new();
  clap_complete::generate(
    clap_complete_fig::Fig,
    &mut SnmCli::command(),
    "snm",
    &mut output,
  );
  let mut output_string = String::from_utf8(output).unwrap();

  output_string = output_string.replace("const completion: Fig.Spec = {", "const completion = {");

  if let Some(home) = dirs::home_dir() {
    let dir = home.join(".fig").join("autocomplete").join("build");

    if dir.exists().not() {
      fs::create_dir_all(&dir)
        .expect(format!("fig_spec_impl create_dir_all error {:?}", &dir.display()).as_str());
    }

    let spec_path_buf = dir.join("snm.js");

    if spec_path_buf.exists() {
      fs::remove_file(&spec_path_buf).expect(
        format!(
          "fig_spec_impl remove_file error {:?}",
          &spec_path_buf.display()
        )
        .as_str(),
      );
    }

    fs::write(&spec_path_buf, &output_string)?;

    let message = format!(
      r##"
    🎉 Fig spec file create successful. 

    🔔 Now ! Fig rename to {}

                                    {}
            "##,
      "Amazon Q".green().bold(),
      "Powered by snm".bright_black(),
    );

    eprintln!("{message}");
  }

  Ok(())
}

fn setup_symlink() -> anyhow::Result<()> {
  let exe = current_exe()?;
  let exe_dir = exe.parent().unwrap();

  const SHIM_TARGETS: &[&str] = &["npm", "npx", "yarn", "pnpm", "pnpx", "node"];

  #[cfg(windows)]
  let source = exe_dir.join("snm-shim.exe");
  #[cfg(not(windows))]
  let source = exe_dir.join("snm-shim");

  // let source = exe_dir.join("snm-shim");
  for target in SHIM_TARGETS {
    // let target = exe_dir.join(target);
    #[cfg(windows)]
    let target = exe_dir.join(format!("{}.exe", target));
    #[cfg(not(windows))]
    let target = exe_dir.join(target);
    if target.try_exists()? {
      std::fs::remove_file(&target)?;
    }
    #[cfg(unix)]
    {
      std::os::unix::fs::symlink(&source, &target)?;
    }
    #[cfg(windows)]
    {
      std::os::windows::fs::symlink_file(&source, &target)?;
    }
  }

  Ok(())
}
