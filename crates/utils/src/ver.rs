use semver::{Version, VersionReq};

pub fn ver_gt_1(version: &str) -> anyhow::Result<bool> {
  let version = Version::parse(version)?;
  let req = VersionReq::parse(">1")?;
  Ok(req.matches(&version))
}
