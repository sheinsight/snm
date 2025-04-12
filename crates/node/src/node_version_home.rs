use std::path::PathBuf;

pub struct NodeVersionHome(pub PathBuf);

impl NodeVersionHome {
  pub fn exe(&self) -> PathBuf {
    let dir = &self.0;
    if cfg!(windows) {
      dir.join("node.exe")
    } else {
      dir.join("bin").join("node")
    }
  }

  pub fn bin_dir(&self) -> PathBuf {
    let dir = &self.0;
    if cfg!(windows) {
      dir.to_owned()
    } else {
      dir.join("bin")
    }
  }
}
