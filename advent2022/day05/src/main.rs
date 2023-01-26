use std::{fs, vec};
fn main() {
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    // let mut test_data = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
    let mut part_1 = vec![
        vec!["R", "S", "L", "F", "Q"],
        vec!["N", "Z", "Q", "G", "P", "T"],
        vec!["S", "M", "Q", "B"],
        vec!["T", "G", "Z", "J", "H", "C", "B", "Q"],
        vec!["P", "H", "M", "B", "N", "F", "S"],
        vec!["P", "C", "Q", "N", "S", "L", "V", "G"],
        vec!["W", "C", "F"],
        vec!["Q", "H", "G", "Z", "W", "V", "P", "M"],
        vec!["G", "Z", "D", "L", "C", "N", "R"]
    ];
    let mut part_2 = part_1.clone();

    // Part 1
    for line in data.split("\n") {
        if line.is_empty() {
            break;
        }
        let ((from, to), amount) = extract_commands(line);
        let mut to_move: Vec<&str> = vec![];
        for _ in 0..amount {
            let popped = part_1[from].pop().unwrap();
            to_move.push(popped);
        }
        for item in to_move.iter() {
            part_1[to].push(item);
        }
    }

    // Part 2
    for line in data.split("\n") {
        if line.is_empty() {
            break;
        }
        let ((from, to), amount) = extract_commands(line);
        let length = part_2[from].len();
        let to_keep = part_2[from][0..length-amount].to_owned();
        let to_move = part_2[from][length-amount..].to_owned();
        part_2[from] = to_keep;
        for item in to_move {
            part_2[to].push(item)
        }
    }
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

fn extract_commands(line: &str) -> ((usize,usize), usize) {
    let splits: Vec<&str> = line.split(" ").collect();
    let amount = splits[1].parse::<usize>().unwrap();
    let from: usize = splits[3].parse::<usize>().unwrap() - 1;
    let to: usize = splits[5].parse::<usize>().unwrap() - 1;
    ((from, to), amount)
}
