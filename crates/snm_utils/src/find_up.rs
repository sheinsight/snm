use std::path::{Path, PathBuf};

pub struct FindUp<P: AsRef<Path>> {
  pub cwd: P,
}

impl<P: AsRef<Path>> FindUp<P> {
  pub fn new(cwd: P) -> Self {
    Self { cwd }
  }

  pub fn find(&self, name: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut res = vec![];
    let mut cwd = self.cwd.as_ref();
    let file = cwd.join(name);

    if file.exists() {
      res.push(file);
    }

    while let Some(dir) = cwd.parent() {
      let file = dir.join(name);
      if file.exists() {
        res.push(file);
      }
      cwd = dir;
    }

    Ok(res)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_find_up() {
    let find_up = FindUp::new("./fixtures/a/b/c/d");
    let res = find_up.find(".node-version").unwrap();
    assert_eq!(res.len(), 2);
  }
}
