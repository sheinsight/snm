use std::io::{stdout, Write};

use snm_core::print_warning;

pub fn progress(_downloaded_size: u64, _total_size: u64) {
    let mut stdout = stdout();

    print_warning!(stdout, "Download waiting...");
    let _ = std::io::stdout().flush();
}
