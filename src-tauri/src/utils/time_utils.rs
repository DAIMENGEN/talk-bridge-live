use chrono::{Local, DateTime};
pub fn format_local_datetime(dt: DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}