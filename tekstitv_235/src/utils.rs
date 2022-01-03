pub fn is_empty_or_whitespace(line: &str) -> bool {
    line.is_empty() || line.chars().all(|c| c.is_whitespace())
}
