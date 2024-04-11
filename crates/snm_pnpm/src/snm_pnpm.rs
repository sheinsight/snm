use snm_npm::snm_npm::SnmNpm;

pub struct SnmPnpm;

impl SnmPnpm {
    pub fn new() -> SnmNpm {
        SnmNpm::from_prefix("pnpm")
    }
}
