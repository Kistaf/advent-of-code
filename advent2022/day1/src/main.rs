use std::fs;

fn main() {
    // Read the entirety of the file at path specified
    let path: &str = "data.txt";
    let data = fs::read_to_string(path).expect(&format!("Failed to load {}", path));

    // split but double space
    let counts = data.split("\n\n").map(|rows| -> usize {
        // sum all single lines within a double newline area
        rows.split("\n").map(|row| row.parse().unwrap_or(0)).sum()
    });

    // convert to vec (mut because we will sort afterwards, and sort is
    // in-place, therefore we need to be able to change the data)
    let mut v: Vec<_> = counts.collect();

    // sort it
    v.sort();

    // we can assume the highest count is the last index as the vec is sorted
    let last_idx = v.len() - 1;

    // print result
    println!("The highest count is: {}", v[last_idx]);

    /*
        part 2
    */

    / Splits the collection into two at the given index.
    // We are returned the new collection, which we can iterate over and sum up
    let top_3: usize = v.split_off(v.len() - 3).into_iter().sum();
    println!("The sum of the top 3 is: {}", top_3);
}
