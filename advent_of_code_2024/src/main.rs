// use std::env;
use std::fs;

fn main() {
    day_one();
}

fn day_one() {
    // File path
    let file_path = "input/day_one.txt";

    // Read the file
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read the file");

    // Split to vector
    let nums: Vec<&str> = contents
        .split(" ")
        .collect();

    for num in nums {
        println!("{}", num);
    }
}
