#[derive(Debug)]
pub struct Flag {
  pub condition: bool,
  pub flag: &'static str,
}

impl Flag {
  pub fn new(condition: bool, flag: &'static str) -> Self {
    Self { condition, flag }
  }

  pub fn is_active(&self) -> bool {
    self.condition
  }

  pub fn to_string_if_active(&self) -> Option<String> {
    self.condition.then(|| self.flag.to_string())
  }
}
