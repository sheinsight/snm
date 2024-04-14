mod shim;

use crate::shim::launch_shim;
use semver::Version;
use snm_core::model::{trait_manage::ManageTrait, trait_shim::ShimTrait, SnmError};
use snm_yarn::{snm_yarn::SnmYarn, snm_yarnpkg::SnmYarnPkg};

#[tokio::main]
async fn main() {
    exec().await.unwrap();
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
