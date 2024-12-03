use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_file = File::open("../input.txt").expect("failed to open file");
    let reader = BufReader::new(input_file);

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("failed to read line");
        let mut values = line.split_whitespace();
        let first = values
            .next()
            .unwrap_or_else(|| panic!("first value is not provided on line {i}"))
            .parse::<u32>()
            .unwrap();
        first_list.push(first);

        let second = values
            .next()
            .unwrap_or_else(|| panic!("second value is not provided on line {i}"))
            .parse::<u32>()
            .unwrap();
        second_list.push(second);
    }

    first_list.sort_unstable();
    second_list.sort_unstable();

    let mut distances: Vec<u32> = vec![];
    for (i, first) in first_list.iter().enumerate() {
        let second = second_list.get(i).unwrap();
        distances.push(first.abs_diff(*second));
    }

    println!("{}", distances.iter().sum::<u32>());
}
