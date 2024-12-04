#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    let mut left_column:Vec<i32> = Vec::new();
    let mut right_column:Vec<i32> = Vec::new();

    for line in lines {
        // Split the line into two parts
        let line_parts: Vec<i32> = line.split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
        left_column.push(line_parts[0]);
        right_column.push(line_parts[1]);
    }

    left_column.sort();
    right_column.sort();

    while left_column.len() > 0{
        let left = left_column.pop().unwrap();
        let right = right_column.pop().unwrap();
        result += (right - left).abs();
    }


    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, "11");
    }
}
