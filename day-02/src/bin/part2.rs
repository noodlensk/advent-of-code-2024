#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;
    for line in lines {
        let line_parts: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if line_parts.len() == 0 {
            continue;
        }

        let is_safe_init = is_safe(line_parts.clone());
        if is_safe_init {
            sum += 1;

            continue;
        }

        for i in 0..line_parts.len() {
            let mut line_parts_copy = line_parts.clone();
            line_parts_copy.remove(i);

            if is_safe(line_parts_copy) {
                sum += 1;
                break;
            }
        }

    }
    sum.to_string()
}

pub fn is_safe(input: Vec<u32>) -> bool {
    let increasing = (input[0] as i32 - input[1] as i32) < 0;

    let mut is_safe = true;

    for i in 1..input.len() {
        let diff = input[i - 1] as i32 - input[i] as i32;

        if diff == 0 {
            is_safe = false;
        }

        if diff.abs() > 3 {
            is_safe = false;
        }

        if increasing && diff > 0 {
            is_safe = false;
        }

        if !increasing && diff < 0 {
            is_safe = false;
        }

        if !is_safe {
            break;
        }
    }

    is_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "4");
    }
}
