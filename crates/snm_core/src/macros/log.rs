#[macro_export]
macro_rules! println_error {
    ($out:expr, $($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };
        execute!($out, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        writeln!($out, "{} 🔴 {}", prefix, format_args!($($arg)*)).ok();
        $out.flush().ok();
    }};
}

#[macro_export]
macro_rules! println_success {
    ($out:expr, $($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };
        execute!($out, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();
        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        writeln!($out, "{} 🟢 {}", prefix, format_args!($($arg)*)).ok();
        $out.flush().ok();
    }};
}

#[macro_export]
macro_rules! print_warning {
    ($out:expr, $($arg:tt)*) => {{
        use std::io::Write; // 导入 Write trait 以使用 write! 宏
        use $crate::crossterm::{
            execute,
            cursor::MoveToColumn,
            terminal::{Clear, ClearType},
        };

        // 假设你想在打印警告之前清除当前行并将光标移动到行首
        // 这需要crossterm或类似库的支持
        execute!($out, Clear(ClearType::CurrentLine), MoveToColumn(0)).ok();

        let prefix = format!("\x1B[96m[SNM]\x1B[0m");
        write!($out, "{} 🟡 {}", prefix, format_args!($($arg)*)).ok();

        $out.flush().ok();
    }};
}
