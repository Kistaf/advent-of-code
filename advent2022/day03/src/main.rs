use std::fs;

fn main() {
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    let priority: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut lines: Vec<&str> = Vec::new(); // used for part 2
    let total: usize = data
        .split("\n")
        .map(|line| -> usize {
            lines.push(line); // part 2 preparation
            match find_common(line) {
                Some(c) => priority.find(c).unwrap_or(0) + 1,
                None => 0,
            }
        })
        .sum();

    // Part 2
    let mut sum: i32 = 0;
    for i in (2..lines.len() - 1).step_by(3) {
        for letter in lines[i].chars() {
            if lines[i - 1].contains(letter) && lines[i - 2].contains(letter) {
                sum += match priority.find(letter) {
                    Some(r) => r + 1,
                    None => 0,
                } as i32;
                break;
            }
        }
    }

    // Results
    println!("The total score is: {}", total);
    println!("Total sum: {}", sum);
}

fn find_common(s: &str) -> Option<char> {
    let (l, r) = s.split_at(s.len() / 2);
    for letter in l.chars() {
        if r.contains(letter) {
            return Some(letter);
        }
    }
    return None;
}
