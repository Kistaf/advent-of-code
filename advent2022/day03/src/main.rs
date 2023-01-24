use std::fs;

fn main() {
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    let total: usize = data.split("\n").map(|line| find_common_score(line).unwrap_or(0)).sum();
    println!("The total score is: {}", total)
}

fn find_common_score(s: &str) -> Option<usize> {
    let (l, r) = s.split_at(s.len() / 2);
    for letter in l.chars() {
        if r.contains(letter) {
            let priority: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let score: usize = priority.find(letter)? + 1;
            return Some(score);
        }
    }
    return None;
}
