use log::{Level, LevelFilter, Log, Metadata, Record};
use std::sync::{Arc, Mutex};

/// 共享的日志缓冲区
pub type LogBuffer = Arc<Mutex<String>>;

/// 创建一个新的日志缓冲区
pub fn new_log_buffer() -> LogBuffer {
    Arc::new(Mutex::new(String::new()))
}

/// 自定义 Logger，将日志写入共享缓冲区
struct UiLogger {
    buffer: LogBuffer,
    max_lines: usize,
}

impl Log for UiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Ok(mut buf) = self.buffer.lock() {
                // 格式化日志消息
                let msg = format!(
                    "[{}] {}: {}\n",
                    record.level(),
                    record.target(),
                    record.args()
                );
                buf.push_str(&msg);

                // 限制日志行数，保留最新的
                let lines: Vec<&str> = buf.lines().collect();
                if lines.len() > self.max_lines {
                    let keep = lines.len() - self.max_lines;
                    *buf = lines[keep..].join("\n") + "\n";
                }
            }
        }
    }

    fn flush(&self) {}
}

/// 初始化日志捕获系统
///
/// # Arguments
/// * `buffer` - 共享的日志缓冲区
/// * `max_lines` - 最大保留行数
///
/// # Returns
/// 如果初始化成功返回 Ok，否则返回错误
pub fn init_log_capture(buffer: LogBuffer, max_lines: usize) -> Result<(), log::SetLoggerError> {
    let logger = UiLogger {
        buffer,
        max_lines,
    };

    // 使用 Box::leak 来创建 'static 生命周期的 logger
    let logger = Box::leak(Box::new(logger));

    log::set_logger(logger)?;
    log::set_max_level(LevelFilter::Info);

    Ok(())
}

/// 从缓冲区读取当前日志内容
pub fn get_logs(buffer: &LogBuffer) -> String {
    buffer.lock().map(|buf| buf.clone()).unwrap_or_default()
}

/// 清空日志缓冲区
pub fn clear_logs(buffer: &LogBuffer) {
    if let Ok(mut buf) = buffer.lock() {
        buf.clear();
    }
}

/// 手动追加消息到日志（用于捕获 println! 的输出）
pub fn append_log(buffer: &LogBuffer, message: &str) {
    if let Ok(mut buf) = buffer.lock() {
        buf.push_str(message);
        if !message.ends_with('\n') {
            buf.push('\n');
        }
    }
}
