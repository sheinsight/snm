pub struct SnmNpmrc;

use directories::{self, BaseDirs};
use std::path::PathBuf;

use ini::Ini;
use ordered_multimap::list_ordered_multimap::ListOrderedMultimap;

pub fn load_npmrc(dir: PathBuf) -> ListOrderedMultimap<String, String> {
    let mut all_map: ListOrderedMultimap<String, String> = ListOrderedMultimap::new();
    let mut path_list: Vec<PathBuf> = vec![];
    let mut parent = Some(dir.as_path());
    let base_dirs = BaseDirs::new().unwrap();
    let home_dir = base_dirs.home_dir();

    while let Some(p) = parent {
        if p != home_dir {
            path_list.insert(0, p.to_path_buf());
        }

        if p == dir {
            path_list.insert(0, home_dir.to_path_buf())
        }
        parent = p.parent();
    }

    for p in path_list.iter() {
        let npmrc_file = p.join(".npmrc");
        let conf = Ini::load_from_file(npmrc_file).unwrap_or(Ini::default());

        let section_none: Option<String> = None;

        for i in conf.section(section_none).iter() {
            for (key, value) in i.iter() {
                all_map.insert(key.to_string(), value.to_string());
            }
        }
    }
    all_map
}
