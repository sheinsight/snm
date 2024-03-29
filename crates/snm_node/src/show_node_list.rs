use colored::Colorize;

pub fn show_node_list<F>(node_vec: Vec<snm_core::model::NodeModel>, get_tag_fn: F)
where
    F: Fn(&String) -> &str,
{
    for node in node_vec.iter() {
        let lts = match &node.lts {
            snm_core::model::Lts::Str(s) => s,
            snm_core::model::Lts::Bool(_) => "",
        };

        let deprecated = node.deprecated.unwrap_or(false);

        let version = if deprecated {
            format!(
                "{:<10} {:<10}",
                node.version.bright_black(),
                lts.bright_black()
            )
        } else {
            format!(
                "{:<10} {:<10}",
                node.version.bright_green(),
                lts.bright_green()
            )
        };

        let died = format!("died on {}", node.end.as_deref().unwrap_or("")).bright_black();

        let npm = format!("npm {}", node.npm.as_deref().unwrap_or("None")).bright_black();

        let openssl =
            format!("openssl {}", node.openssl.as_deref().unwrap_or("None")).bright_black();

        let desc_width = 22;

        let tag = get_tag_fn(&node.version);

        // 标记
        println!(
            "{:<2} {} {:<desc_width$} {:<desc_width$} {:<desc_width$}",
            tag, version, died, openssl, npm,
        );
    }
}
