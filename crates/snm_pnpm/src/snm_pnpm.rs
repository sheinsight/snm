use snm_package_manager::snm_package_manager::SnmPackageManager;

pub struct SnmPnpm;

impl SnmPnpm {
    pub fn new() -> SnmPackageManager {
        SnmPackageManager::from_prefix("pnpm")
    }
}
