#[macro_export]
macro_rules! trace_if {
    // 闭包形式，直接执行闭包内的代码
    (|| $($body:tt)*) => {
        if tracing::enabled!(tracing::Level::TRACE) {
            $($body)*
        }
    };
    // 原始形式
    ($($arg:tt)*) => {
        if tracing::enabled!(tracing::Level::TRACE) {
            trace!($($arg)*);
        }
    };
}
