use std::path::Path;

use anyhow::bail;
use colored::Colorize;
use package_json_parser::PackageJsonParser;
use semver::Version;
use snm_config::snm_config::SnmConfig;
use snm_package_manager::{PackageManager, PackageManagerKind};
use snm_utils::exec::{exec_cli, exec_cli_with_envs};

pub struct PmShim {
  pub args: Vec<String>,
  pub paths: Vec<String>,
  pub snm_config: SnmConfig,
}

impl PmShim {
  pub fn new(args: Vec<String>, paths: Vec<String>, snm_config: SnmConfig) -> Self {
    Self {
      args,
      paths,
      snm_config,
    }
  }

  pub async fn proxy(&self) -> anyhow::Result<()> {
    let [bin_name, command, args @ ..] = self.args.as_slice() else {
      bail!(r#"deconstruct args failed, args: {:?}"#, self.args);
    };

    let resolver = snm_package_manager::PackageManagerResolver::from(self.snm_config.clone());

    let Some(package_manager) = resolver.find_up_package_manager()? else {
      return exec_cli(
        &[&[bin_name.clone(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      );
    };

    // 传进来的有可能是绝对路径, 如果是绝对路径的的话，取 file_name 判断一下。
    // 同时需要保证直取命令的名称，方便 后续的 json.get_bin_with_name(bin_name) 获取到对应 js 的真实路径
    // 主要用来拦截处理 snm 自己创建的 symlink , windows 下 symlink 拿到的是绝对路径
    let bin_name = if Path::new(bin_name).is_absolute() {
      Path::new(bin_name)
        .file_name()
        .and_then(|f| f.to_str())
        .map(|name| {
          name
            .strip_suffix(".cmd")
            .or_else(|| name.strip_suffix(".exe"))
            .unwrap_or(name)
        })
        .unwrap_or(bin_name)
    } else {
      bin_name
    };

    let matched = &[package_manager.name(), "npx", "pnpx"].contains(&bin_name);
    if !matched {
      bail!(
        "Package manager mismatch, expect: {}, actual: {}",
        package_manager.name().green(),
        bin_name.red()
      );
    }

    let dir = resolver.ensure_package_manager(&package_manager).await?;

    // snm 与 Corepack 一样只解压主包，不安装可选依赖，也不执行 preinstall。
    // pnpm 12 因此必须走官方保留的 .mjs 引导入口，不能执行 package.json#bin 中的占位文件。
    if let Some(entrypoint) = pnpm_corepack_entrypoint(&package_manager, bin_name) {
      let file = dir.join(entrypoint);
      if !file.is_file() {
        bail!(
          "pnpm Corepack entrypoint does not exist: {}",
          file.display()
        );
      }

      // 用户显式设置的 Corepack registry 优先；否则让原生二进制下载沿用 snm 的 registry。
      let corepack_env_overrides = if std::env::var_os("COREPACK_NPM_REGISTRY").is_none() {
        vec![(
          "COREPACK_NPM_REGISTRY",
          self.snm_config.npm_registry.as_str(),
        )]
      } else {
        Vec::new()
      };
      exec_cli_with_envs(
        &[
          &[
            "node".to_string(),
            file.to_string_lossy().into_owned(),
            command.to_owned(),
          ],
          args,
        ]
        .concat(),
        &self.paths,
        true,
        &corepack_env_overrides,
      )?;
      return Ok(());
    }

    let json = PackageJsonParser::parse(dir.join("package.json")).map_err(|e| {
      eprintln!("{:?}", e);
      anyhow::anyhow!("parse package.json failed, err: {:?}", e)
    })?;

    let map = json
      .bin_to_hash_map()
      .map_err(|e| anyhow::anyhow!("{}", e))?;

    if let Some(file) = map.get(bin_name) {
      let file = dir.join(file);
      exec_cli(
        &[
          &[
            "node".to_string(),
            file.to_string_lossy().into_owned(),
            command.to_owned(),
          ],
          args,
        ]
        .concat(),
        &self.paths,
        true,
      )?;
    } else {
      exec_cli(
        &[&[bin_name.to_string(), command.to_owned()], args].concat(),
        &self.paths,
        true,
      )?;
    }

    // if let Ok(file) = json.get_bin_with_name(bin_name) {
    //   exec_cli(
    //     &[
    //       &[
    //         "node".to_string(),
    //         file.to_string_lossy().into_owned(),
    //         command.to_owned(),
    //       ],
    //       args,
    //     ]
    //     .concat(),
    //     &self.paths,
    //     true,
    //   )?;
    // } else {
    //   exec_cli(
    //     &[&[bin_name.to_string(), command.to_owned()], args].concat(),
    //     &self.paths,
    //     true,
    //   )?;
    // }

    Ok(())
  }
}

/// 仅对 pnpm 12+ 的 pnpm/pnpx 命令选择官方 Corepack 入口，其他版本继续走原有 bin 解析。
fn pnpm_corepack_entrypoint(
  package_manager: &PackageManager,
  bin_name: &str,
) -> Option<&'static str> {
  if package_manager.kind() != PackageManagerKind::Pnpm {
    return None;
  }

  let version = Version::parse(package_manager.version()).ok()?;
  if version.major < 12 {
    return None;
  }

  match bin_name {
    "pnpm" => Some("bin/pnpm.mjs"),
    "pnpx" => Some("bin/pnpx.mjs"),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use super::*;

  #[test]
  fn should_only_use_corepack_entrypoints_for_pnpm_12() -> anyhow::Result<()> {
    let pnpm_11 = PackageManager::from_str("pnpm@11.0.0")?;
    let pnpm_12 = PackageManager::from_str("pnpm@12.0.0")?;
    let npm_12 = PackageManager::from_str("npm@12.0.0")?;

    // 版本、包管理器或命令任一不匹配时，都不能改变旧执行链路。
    assert_eq!(pnpm_corepack_entrypoint(&pnpm_11, "pnpm"), None);
    assert_eq!(pnpm_corepack_entrypoint(&npm_12, "npm"), None);
    assert_eq!(pnpm_corepack_entrypoint(&pnpm_12, "npx"), None);
    assert_eq!(
      pnpm_corepack_entrypoint(&pnpm_12, "pnpm"),
      Some("bin/pnpm.mjs")
    );
    assert_eq!(
      pnpm_corepack_entrypoint(&pnpm_12, "pnpx"),
      Some("bin/pnpx.mjs")
    );
    Ok(())
  }
}
