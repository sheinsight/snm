use snm_npm::snm_npm::SnmNpm;

pub struct SnmYarn;

impl SnmYarn {
    pub fn new() -> SnmNpm {
        SnmNpm::from_prefix("yarn")
    }
}
