#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };
        use std::io::stdout;

        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        writeln!(stdout, "{} 🔴 {}", prefix, format_args!($($arg)*)).ok();
        stdout.flush().ok();
    }};
}

#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };
        use std::io::stdout;

        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        writeln!(stdout, "{} 🟢 {}", prefix, format_args!($($arg)*)).ok();
        stdout.flush().ok();
    }};
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };
        use std::io::stdout;

        let mut stdout = stdout();
        // 假设你想在打印警告之前清除当前行并将光标移动到行首
        // 这需要crossterm或类似库的支持
        execute!(stdout, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();

        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        write!(stdout, "{} 🟡 {}", prefix, format_args!($($arg)*)).ok();

        stdout.flush().ok();
    }};
}
