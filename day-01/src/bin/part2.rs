#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: i32 = 0;

    let mut left_column:Vec<i32> = Vec::new();
    let mut right_column:Vec<i32> = Vec::new();

    for line in lines {
        // Split the line into two parts
        let line_parts: Vec<i32> = line.split("   ").map(|x| x.parse::<i32>().unwrap()).collect();
        left_column.push(line_parts[0]);
        right_column.push(line_parts[1]);
    }

    for left in left_column.iter() {
      let cnt = right_column.iter().filter(|&&right| right.eq(left)).count() as i32;

        sum = sum + cnt * left;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, "31");
    }
}
