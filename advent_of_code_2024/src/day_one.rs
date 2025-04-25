use std::io::BufRead;
use std::fs::File;
use std::io;

pub fn day_one() {
    // File path
    let file_path = "input/day_one.txt";
    let file = File::open(&file_path).expect("File not found");
    let reader = io::BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    // Read the file and put numbers in their own vectors
    for line in reader.lines() {
        let line = line.expect("Could not read file");
        let split: Vec<&str> = line.split(" ").collect();
        
        left.push(split[0].parse::<i32>().unwrap());
        right.push(split[3].parse::<i32>().unwrap());
    }

    // Sort the vectors
    left.sort();
    right.sort();

    // left and right differences to sum
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    // print our answer
    println!("{}", sum);
}

pub fn day_one_two() {
    // File path
    let file_path = "input/day_one.txt";
    let file = File::open(&file_path).expect("File not found");
    let reader = io::BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    // Read the file and put numbers in their own vectors
    for line in reader.lines() {
        let line = line.expect("Could not read file");
        let split: Vec<&str> = line.split(" ").collect();
        
        left.push(split[0].parse::<i32>().unwrap());
        right.push(split[3].parse::<i32>().unwrap());
    }

    // Sort the vectors
    left.sort();
    right.sort();

    // left and right differences to sum
    for i in left {
        let power: i32 = right.iter().filter(|&n| *n==i).count() as i32;
        sum += i * power;
    }

    // print our answer
    println!("{}", sum);
}
