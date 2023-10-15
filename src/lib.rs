#[derive(Eq, PartialEq, Hash)]
pub enum Level {
    Debug = 1,
    Info  = 2,
    Warn  = 3,
    Error = 4,
}

lazy_static::lazy_static! {
    static ref LEVELS: std::collections::HashMap<Level, &'static str> = {
        let mut map = std::collections::HashMap::new();
        map.insert(Level::Debug, "D");
        map.insert(Level::Info, "I");
        map.insert(Level::Warn, "W");
        map.insert(Level::Error, "E");
        map
    };
}

/// 输出调试日志(仅debug编译有效)
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        log::write(log::Level::Debug, format!($($arg)*));
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{}}; // release模式完全不编译
}

/// 输出信息日志
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        log::write(log::Level::Info, format!($($arg)*));
    }};
}

/// 输出警告日志
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        log::write(log::Level::Warn, format!($($arg)*));
    }};
}

/// 输出错误日志
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        log::write(log::Level::Error, format!($($arg)*));
    }};
}

/// 输出日志消息
pub fn write(lv: Level, msg: String) {
    let now = chrono::Local::now();
    println!(
        "[{}.{:03}{} {}] {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        now.timestamp_subsec_millis(),
        now.format("%z"),
        LEVELS[&lv],
        msg
    );
}