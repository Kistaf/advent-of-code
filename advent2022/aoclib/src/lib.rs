use std::fs;

pub fn read_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&format!("Unable to read path: {}", path))
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
