use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

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

pub fn init_snm_log() -> anyhow::Result<()> {
  if let Some(home) = dirs::home_dir() {
    let file = std::fs::File::create(home.join("snm.log"))?;

    let file_layer = fmt::layer()
      .with_writer(file)
      .with_filter(EnvFilter::from_env("SNM_LOG"));

    // 创建控制台写入器
    let stdout_layer = fmt::layer()
      .with_file(true)
      .with_line_number(true)
      .with_span_events(fmt::format::FmtSpan::FULL)
      .with_target(true)
      .with_filter(EnvFilter::from_env("SNM_LOG"));

    tracing_subscriber::registry()
      .with(file_layer)
      .with(stdout_layer)
      .try_init()?;
  }
  Ok(())
}
