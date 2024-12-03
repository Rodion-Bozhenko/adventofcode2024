use std::fs::read_to_string;

fn main() {
    println!("DISTANCE: {}", part_one("../input.txt"));
    println!("SIMILARITY_SCORE: {}", part_two("../input.txt"));
}

fn part_one(path: &str) -> u32 {
    let (mut first_list, mut second_list) = parse(path);

    first_list.sort_unstable();
    second_list.sort_unstable();

    let mut distances: Vec<u32> = vec![];
    for (i, first) in first_list.iter().enumerate() {
        let second = second_list.get(i).unwrap();
        distances.push(first.abs_diff(*second));
    }

    return distances.iter().sum::<u32>();
}

fn part_two(path: &str) -> u32 {
    let (first_list, second_list) = parse(path);
    first_list
        .iter()
        .map(|first| {
            let count = second_list.iter().filter(|second| *second == first).count();
            first * count as u32
        })
        .sum()
}

fn parse(path: &str) -> (Vec<u32>, Vec<u32>) {
    let input_file = read_to_string(path).expect("failed to open file");

    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();

    for (i, line) in input_file.lines().enumerate() {
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

    (first_list, second_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(part_one("../input-test.txt"), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(part_two("../input-test.txt"), 31);
    }
}
