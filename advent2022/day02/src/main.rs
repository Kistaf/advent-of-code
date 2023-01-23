use std::collections::HashMap;
use std::fs;

fn main() {
    let scores_part1 = HashMap::from([
        ("AX", 4),
        ("AY", 8),
        ("AZ", 3),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 7),
        ("CY", 2),
        ("CZ", 6),
    ]);
    let scores_part2 = HashMap::from([
        ("AX", 3),
        ("AY", 4),
        ("AZ", 8),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 2),
        ("CY", 6),
        ("CZ", 7),
    ]);
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    let part_1 = calculate_score(&data, &scores_part1);
    let part_2 = calculate_score(&data, &scores_part2);
    println!("Total count: {}", part_1);
    println!("Total count: {}", part_2);
}

fn calculate_score(data: &String, hash: &HashMap<&str, i32>) -> i32 {
    data.split("\n")
        .map(|row| hash.get(row.replace(" ", "").as_str()).unwrap_or(&0))
        .sum()
}
