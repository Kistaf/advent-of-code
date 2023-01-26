use aoclib::read_lines;

fn main() {
    let lines = read_lines("data.txt");
    // let path: &str = "data.txt";
    // let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));
    let mut occurences_part1: i32 = 0;
    let mut occurences_part2: i32 = 0;
    for line in lines {
        // Extract each half and convert their respective ranges to individual i32's in tuple
        // format using Vec
        let ranges = line
            .split(',')
            .map(|half| {
                let range = half
                    .split('-')
                    .map(|r| r.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                (range[0], range[1])
            })
            .collect::<Vec<(i32, i32)>>();

        // Part 1
        if ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1
            || ranges[1].0 <= ranges[0].0 && ranges[1].1 >= ranges[0].1
        {
            occurences_part1 += 1;
        }

        // Part 2
        if ranges[0].0 <= ranges[1].0 && ranges[0].1 >= ranges[1].1
            || ranges[1].0 <= ranges[0].0 && ranges[1].1 >= ranges[0].1
            || ranges[0].0 >= ranges[1].0 && ranges[0].0 <= ranges[1].1
            || ranges[0].1 >= ranges[1].0 && ranges[0].1 <= ranges[1].1
        {
            occurences_part2 += 1;
        }
    }
    println!(
        "Number of assignments which fully contain the other: {}",
        occurences_part1
    );
    println!("Number of overlaps: {}", occurences_part2);
}
