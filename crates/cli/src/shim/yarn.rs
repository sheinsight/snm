mod shim;

use crate::shim::launch_shim;
use semver::Version;
use shim::check;
use snm_core::model::{
    snm_error::handle_snm_error, trait_manage::ManageTrait, trait_shim::ShimTrait, SnmError,
};
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};

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
    let x: Box<dyn ShimTrait> = Box::new(SnmYarn::new());

    let v = x.get_strict_shim_version()?;

    let less = get_is_less_2(v.as_str())?;

    let instance: Box<dyn ManageTrait> = if less {
        Box::new(SnmYarn::new())
    } else {
        Box::new(SnmYarnPkg::new())
    };

    launch_shim(instance).await;

    Ok(())
}

fn get_is_less_2(v: &str) -> Result<bool, SnmError> {
    let ver = Version::parse(v)?;
    let is_less_2 = ver < Version::parse("2.0.0")?;
    Ok(is_less_2)
}
