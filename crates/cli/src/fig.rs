use std::{fs, ops::Not};

use clap::CommandFactory;
use snm_core::println_success;

use crate::SnmCli;

pub fn fig_spec_impl() {
    let mut output = Vec::new();
    clap_complete::generate(
        clap_complete_fig::Fig,
        &mut SnmCli::command(),
        "snm",
        &mut output,
    );
    let mut output_string = String::from_utf8(output).unwrap();

    output_string = output_string.replace("const completion: Fig.Spec = {", "const completion = {");

    if let Some(home) = dirs::home_dir() {
        let dir = home.join(".fig").join("autocomplete").join("build");

        if dir.exists().not() {
            fs::create_dir_all(&dir).expect(
                format!("fig_spec_impl create_dir_all error {:?}", &dir.display()).as_str(),
            );
        }

        let spec_path_buf = dir.join("snm.js");

        if spec_path_buf.exists() {
            fs::remove_file(&spec_path_buf).expect(
                format!(
                    "fig_spec_impl remove_file error {:?}",
                    &spec_path_buf.display()
                )
                .as_str(),
            );
        }

        fs::write(&spec_path_buf, &output_string)
            .expect(format!("fig_spec_impl write error {:?}", &spec_path_buf.display()).as_str());

        println_success!(
            "Fig spec file has been created at {}",
            spec_path_buf.display()
        );
    }
}
