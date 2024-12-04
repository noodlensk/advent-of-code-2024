#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: u32 = 0;

    for line in lines {
        let line_parts: Vec<u32> = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        if line_parts.len() == 0 {
            continue;
        }

        let increasing = (line_parts[0] as i32 - line_parts[1] as i32) < 0;

        let mut is_safe = true;

        for i in 1..line_parts.len() {
            let diff = line_parts[i - 1] as i32 - line_parts[i] as i32;

            if diff == 0 {
                is_safe = false;

                break;
            }

            if diff.abs() > 3 {
                is_safe = false;

                break;
            }

            if increasing && diff > 0 {
                is_safe = false;

                break;
            }

            if !increasing && diff < 0 {
                is_safe = false;

                break;
            }
        }

        if is_safe {
            sum += 1;
        }
    }


    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, "2");
    }
}
