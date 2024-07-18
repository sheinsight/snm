use colored::*;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSchedule {
    pub start: String,
    pub end: String,
    pub maintenance: Option<String>,
    pub lts: Option<String>,
    pub codename: Option<String>,
    pub version: Option<String>,
}

impl fmt::Display for NodeSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lts = self
            .lts
            .as_deref()
            .map_or_else(|| format!(""), |lts| format!("Lts by {:<12}", lts));

        let codename = self.codename.as_deref().map_or_else(
            || format!("{:<20}", ""),
            |codename| format!("{:<10} {:<10}", codename.bright_black(), lts.bright_black()),
        );

        write!(
            f,
            "Create by: {:<12}, Death by: {:<12}, Maintenance By {:<12}, Version {:<5} {}",
            self.start.bright_green(),
            self.end.bright_magenta(),
            self.maintenance.as_deref().unwrap_or("none").bright_green(),
            self.version.as_deref().unwrap_or("none").bright_green(),
            codename,
        )
    }
}
