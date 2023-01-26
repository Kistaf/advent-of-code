use aoclib::read_lines;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("data.txt");
    // find marker
    match find(&lines, 4) {
        Some(v) => println!("Characters to find marker: {}", v),
        None => println!("Could not if marker"),
    }
    // find message
    match find(&lines, 14) {
        Some(v) => println!("Characters to find message: {}", v),
        None => println!("Could not if message"),
    }
}

fn find(data: &Vec<String>, marker_length: usize) -> Option<usize> {
    for i in 0..data[0].len() {
        let interval: &str = &data[0][i..i + marker_length];
        let result = is_marker(interval);
        if result {
            return Some(i + marker_length);
        }
    }
    return None;
}

fn is_marker(chunk: &str) -> bool {
    let mut set: HashMap<char, usize> = HashMap::new();
    for character in chunk.chars() {
        if set.contains_key(&character) {
            return false;
        }
        set.insert(character, 1);
    }
    return true;
}

#[cfg(test)]
#[test]
fn part_1_test_1() {
    assert_eq!(
        find(&vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()], 4).unwrap(),
        7
    );
}

#[test]
fn part_1_test_2() {
    assert_eq!(
        find(&vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()], 4).unwrap(),
        5
    );
}

#[test]
fn part_2_test_1() {
    assert_eq!(find(&read_lines("data.txt"), 14).unwrap(), 2263);
}
