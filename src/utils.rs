use std::time::{SystemTime, UNIX_EPOCH};

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn datetime_now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}