use std::fs;
fn main() {
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    // let mut test_data = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
    let mut input_data = vec![
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

    for line in data.split("\n") {
        if line.is_empty() {
            break;
        }
        let splits: Vec<&str> = line.split(" ").collect();
        let amount = splits[1].parse::<usize>().unwrap_or(0);
        let from = splits[3].parse::<usize>().unwrap() - 1;
        let to = splits[5].parse::<usize>().unwrap() - 1;
        let mut to_move: Vec<&str> = vec![];
        for _ in 0..amount {
            let popped = input_data[from].pop().unwrap();
            to_move.push(popped);
        }
        for item in to_move.iter() {
            input_data[to].push(item);
        }
    }

    println!("{:?}", input_data)

}
