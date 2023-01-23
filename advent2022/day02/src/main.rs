use std::collections::HashMap;
use std::fs;

// A - Rock
// B - Paper
// C - Scissors
//
// X - Rock
// Y - Paper
// Z - Scissors
//
// Rock - 1
// Paper - 2
// Scissors - 3
//
// lost - 0
// draw - 3
// win - 6

fn main() {
    let scores = HashMap::from([
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
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    let count: i32 = data
        .split("\n")
        .map(|row| scores.get(row.replace(" ", "").as_str()).unwrap_or(&0))
        .sum();
    println!("Total count: {}", count)
}
