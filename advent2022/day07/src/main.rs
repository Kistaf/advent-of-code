use aoclib::read_lines;


fn main() {
    let lines = read_lines("data.txt");
    let curr_directory_sizes = solve_part_1(lines);
    let part_1: usize = curr_directory_sizes.iter().filter(|x| x < &&100000).sum();
    println!("Part 1: {}", part_1);
    println!("Part 2: {:?}", solve_part_2(curr_directory_sizes));
}

fn solve_part_2(dirs: Vec<usize>) -> usize {
    let size: usize = dirs.last().unwrap().to_owned();
    let max_space = 70000000;
    let required = 30000000;
    let unused = max_space - size;
    let mut curr_best = &required;
    for i in dirs.iter() {
        if (i + unused) > required && i < &curr_best {
            curr_best = i;
        }
    }
    return curr_best.to_owned();
}

fn solve_part_1(lines: Vec<String>) -> Vec<usize> {
    let mut stack: Vec<usize> = vec![];
    let mut totals: Vec<usize> = vec![];
    for line in lines.iter() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match (parts[0], parts[1]) {
            ("$", "ls") => {},
            ("dir", _) => {},
            ("$", "cd") => match parts[2] {
                ".." => {
                    let pop = stack.pop().unwrap();
                    totals.push(pop);
                    if let Some(new_curr) = stack.last_mut() {
                        *new_curr += pop;
                    }
                }
                _ => stack.push(0),
            },
            (size, _) => {
                if let Some(curr) = stack.last_mut() {
                    *curr += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    // empty stack of possible left-over directories
    while let Some(v) = stack.pop() {
        if let Some(top) = stack.last_mut() {
            *top += v;
        }
        totals.push(v);
    }
    return totals;
}

