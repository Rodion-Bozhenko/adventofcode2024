use std::fs::read_to_string;
fn main() {
    println!("RESULT: {}", part_one("../input.txt"));
}

fn part_one(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    input
        .match_indices("mul(")
        .map(|(match_index, _)| {
            let post_mul_pos = match_index + 4;
            let remaining_input = &input[post_mul_pos..];
            let mut comma_pos: usize = 0;
            let mut first_number = "";
            let mut second_number = "";
            for (i, char) in remaining_input.chars().enumerate() {
                if char == ',' && first_number.is_empty() {
                    comma_pos = i;
                    first_number = &remaining_input[..i];
                    continue;
                }
                if comma_pos != 0 && char == ')' && second_number.is_empty() {
                    second_number = &remaining_input[comma_pos + 1..i];
                    if let Ok(first) = first_number.parse::<u32>() {
                        if let Ok(second) = second_number.parse::<u32>() {
                            return first * second;
                        }
                    }
                    break;
                }

                if i > 7 {
                    break;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(161, part_one("../input-test.txt"));
    }
}
