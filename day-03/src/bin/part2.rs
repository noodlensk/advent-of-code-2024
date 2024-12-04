#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let max_mul_instruction_length: usize = "mul(123,123)".len().max("don't()".len());
    let min_mul_instruction_length: usize = "mul(1,1)".len().min("do()".len());
    let mut sum: u32 = 0;

    let mut sum_enabled = true;

    for i in 0..input.len(){
        if i + min_mul_instruction_length > input.len() {
            break;
        }

        let mut end = i + max_mul_instruction_length;
        if end > input.len() {
            end = input.len();
        }

        let mut string_to_check = &input[i..end];

        dbg!(i, string_to_check);

        if string_to_check.starts_with("don't()") {
            sum_enabled = false;
            continue;
        }

        if string_to_check.starts_with("do()") {
            sum_enabled = true;
            continue;
        }

        if !string_to_check.starts_with("mul(") {
            continue;
        }


        string_to_check = string_to_check.trim_start_matches("mul(");

        let del_idx = string_to_check.find(",");
        if del_idx.is_none() {
            continue;
        }

        let x = string_to_check[..del_idx.unwrap()].parse::<u32>();
        if x.is_err() {
            continue;
        }

        let string_to_check = &string_to_check[del_idx.unwrap()+1..];
        let del_idx = string_to_check.find(")");
        if del_idx.is_none() {
            continue;
        }

        let y = string_to_check[..del_idx.unwrap()].parse::<u32>();
        if y.is_err() {
            continue;
        }

        dbg!(i, sum_enabled, x.clone().unwrap(), y.clone().unwrap());


        if sum_enabled {
            sum += x.unwrap() * y.unwrap();
        }
    }


    sum.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48");
    }
}
