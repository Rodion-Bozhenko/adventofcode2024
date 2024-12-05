use std::fs::read_to_string;

fn main() {
    println!("SAFE_REPORTS: {}", part_one("../input.txt"));
    println!(
        "SAFE_REPORTS_WITH_PROBLEM_DAMPENER: {}",
        part_two("../input.txt")
    );
}

fn part_one(path: &str) -> u32 {
    let reports = parse(path);

    reports
        .iter()
        .map(|r| {
            for i in 0..r.len() - 1 {
                let asc = r[1] > r[0];
                let diff = r[i] - r[i + 1];

                if !asc && !(1..=3).contains(&diff) || asc && !(-3..=-1).contains(&diff) {
                    return 0;
                }
            }
            1
        })
        .sum()
}

fn part_two(path: &str) -> u32 {
    let mut reports = parse(path);

    reports
        .iter_mut()
        .map(|report| {
            for i in 0..report.len() {
                let mut report = report.clone();
                report.remove(i);
                if is_report_safe(&report) {
                    return 1;
                }
            }

            0
        })
        .sum()
}

fn is_report_safe(report: &[i32]) -> bool {
    let asc = report[1] > report[0];
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];

        if !asc && !(1..=3).contains(&diff) || asc && !(-3..=-1).contains(&diff) {
            return false;
        }
    }
    true
}

fn parse(path: &str) -> Vec<Vec<i32>> {
    let input_file = read_to_string(path).expect("failed to read input file");
    input_file
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(2, part_one("../input-test.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(4, part_two("../input-test.txt"));
    }
}
