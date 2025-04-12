#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!("{} {}", "[TALK-BRIDGE-LIVE-BACKEND]", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!("{} {}", "[TALK-BRIDGE-LIVE-BACKEND]", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!("{} {}", "[TALK-BRIDGE-LIVE-BACKEND]", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!("{} {}", "[TALK-BRIDGE-LIVE-BACKEND]", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::trace!("{} {}", "[TALK-BRIDGE-LIVE-BACKEND]", format!($($arg)*));
    };
}