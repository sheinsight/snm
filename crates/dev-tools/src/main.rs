use std::{
    fs::File,
    io::{Read as _, Write as _},
};

use glob::glob;
use toml_edit::{value, DocumentMut};

fn main() {
    for entry in glob("crates/**/Cargo.toml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut file = File::open(&path).expect("msg");
                let mut toml_str = String::new();
                file.read_to_string(&mut toml_str).expect("msg");

                let mut doc = toml_str.parse::<DocumentMut>().expect("msg");

                doc["package"]["version"] = value("0.0.1-85");

                let mut file = File::create(&path).expect("msg");
                file.write_all(doc.to_string().as_bytes()).expect("msg");

                file.flush().expect("msg");

                println!("{:?} version changed", path.display())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
