use std::{env::current_dir as std_current_dir, path::PathBuf};

fn truncate_before_node_modules(path: PathBuf) -> PathBuf {
    let mut truncated_path = PathBuf::new();

    for component in path.components() {
        if component.as_os_str() == "node_modules" {
            break;
        }
        truncated_path.push(component);
    }

    truncated_path
}

pub fn current_dir() -> PathBuf {
    let current_dir_path_buf = match std_current_dir() {
        Ok(current_dir_path_buf) => current_dir_path_buf,
        Err(_) => todo!(),
    };

    if !contains(&current_dir_path_buf, "node_modules") {
        return current_dir_path_buf;
    } else {
        return truncate_before_node_modules(current_dir_path_buf);
    }
}

fn contains(path: &PathBuf, target_component: &str) -> bool {
    for component in path.components() {
        if component.as_os_str() == target_component {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_truncate_before_node_modules() {
        assert_eq!(
            truncate_before_node_modules(PathBuf::from("/a/b/c/node_modules/d/e/f")),
            PathBuf::from("/a/b/c")
        );
    }

    #[test]
    fn test_truncate_before_node_modules_when_has_two_node_modules() {
        assert_eq!(
            truncate_before_node_modules(PathBuf::from(
                "/a/b/c/node_modules/d/e/f/node_modules/f/s/e"
            )),
            PathBuf::from("/a/b/c")
        );
    }

    #[test]
    fn test_contains() {
        assert_eq!(
            contains(&PathBuf::from("/a/b/c/node_modules/d/e/f"), "node_modules"),
            true
        );
        assert_eq!(
            contains(&PathBuf::from("/a/b/c/d/e/f"), "node_modules"),
            false
        );
    }

    #[test]
    fn test_current_dir() {
        assert_eq!(current_dir(), std_current_dir().unwrap());
    }

    #[test]
    fn test_current_dir_when_in_node_modules() {
        let current_dir_path_buf = std_current_dir().unwrap();
        let mut node_modules_path_buf = current_dir_path_buf.clone();
        node_modules_path_buf.push("node_modules");
        assert_eq!(
            current_dir(),
            truncate_before_node_modules(node_modules_path_buf)
        );
    }
}
