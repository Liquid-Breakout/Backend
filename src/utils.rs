pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}