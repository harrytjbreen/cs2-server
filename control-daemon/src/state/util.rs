pub fn extract_number_before(line: &str, word: &str) -> Option<u32> {
    line.split(word)
        .next()
        .and_then(|s| s.split_whitespace().last())
        .and_then(|n| n.parse::<u32>().ok())
}

pub fn extract_map_name(line: &str) -> Option<String> {
    // find "de_xxx" inside brackets
    line.split(|c| c == '[' || c == ']' || c == '|')
        .find(|s| s.trim().starts_with("de_"))
        .map(|s| s.trim().to_string())
}
