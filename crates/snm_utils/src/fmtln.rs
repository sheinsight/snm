#[macro_export]
macro_rules! fmtln {
  ($fmt:expr, $($arg:tt)*) => {
      format!($fmt, $($arg)*) + r#"
"#
  };
}
