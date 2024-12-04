#![warn(clippy::pedantic)]
use regex::Regex;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    let email_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(email_pattern).unwrap();
    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();

        let result = x * y;

        sum += result;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, "161");
    }
}
