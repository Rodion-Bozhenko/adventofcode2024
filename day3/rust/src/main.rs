use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").unwrap();
    println!("FIRST_RESULT: {}", part_one(&input));
    println!("SECOND_RESULT: {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
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

fn part_two(input: &str) -> u32 {
    let mut result: u32 = 0;

    let mut token = arrayvec::ArrayString::<11>::new();
    let mut skip = false;

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'm' => token.push(ch),
                'u' if token == *"m" => token.push(ch),
                'l' if token == *"mu" => token.push(ch),
                '(' if token == *"mul" => token.push(ch),
                d if d.is_ascii_digit() && token.starts_with("mul(") => token.push(d),
                ',' if token.starts_with("mul(")
                    && token.chars().skip(4).all(|c| c.is_ascii_digit()) =>
                {
                    token.push(ch);
                }
                ')' if token.starts_with("mul(") && !skip => {
                    if let Some((a, b)) = token[4..].split_once(",") {
                        result += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                    }
                    token.clear();
                }
                'd' => token.push(ch),
                'o' if token == *"d" => {
                    token.push(ch);
                    skip = false;
                }
                'n' if token == *"do" => token.push(ch),
                '\'' if token == *"don" => token.push(ch),
                't' if token == *"don'" => {
                    token.clear();
                    skip = true;
                }
                _ => token.clear(),
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            161,
            part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            48,
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
        )
    }
}
