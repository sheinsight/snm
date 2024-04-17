mod shim;

use std::env::current_dir;

use crate::shim::launch_shim;
use semver::Version;
use shim::check;
use snm_core::model::{
    package_json, snm_error::handle_snm_error, trait_manage::ManageTrait, trait_shim::ShimTrait,
    PackageJson, SnmError,
};
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};

const BIN_NAME: &str = "yarn";

#[tokio::main]
async fn main() {
    match check("yarn") {
        Ok(_) => {
            if let Err(error) = exec().await {
                handle_snm_error(error);
            }
        }
        Err(error) => {
            handle_snm_error(error);
        }
    }
}

async fn exec() -> Result<(), SnmError> {
    let package_json_path_buf = current_dir()
        .expect("get current_dir failed")
        .join("package.json");
    if package_json_path_buf.exists() {
        let x: Box<dyn ShimTrait> = Box::new(SnmYarn::new());
        let v = x.get_strict_shim_version()?;

        let less = get_is_less_2(v.as_str())?;

        let instance: Box<dyn ManageTrait> = if less {
            Box::new(SnmYarn::new())
        } else {
            Box::new(SnmYarnPkg::new())
        };

        launch_shim(instance, BIN_NAME).await;
    } else {
        unimplemented!("yarn unimpl")
    }

    Ok(())
}

fn get_is_less_2(v: &str) -> Result<bool, SnmError> {
    let ver = Version::parse(v).expect(format!("parse version failed {}", &v).as_str());
    let is_less_2 = ver < Version::parse("2.0.0").expect("parse version failed");
    Ok(is_less_2)
}
