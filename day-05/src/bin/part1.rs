#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let rules_part = parts[0].split("\n").collect::<Vec<&str>>();
    let updates_part = parts[1].split("\n").collect::<Vec<&str>>();

    let mut rules: Vec<(u32,u32)> = Vec::new();

    for rule in rules_part {
        let rule_parts = rule.split("|").collect::<Vec<&str>>();
        let rule1 = rule_parts[0].parse::<u32>().unwrap();
        let rule2 = rule_parts[1].parse::<u32>().unwrap();
        rules.push((rule1, rule2));
    }

    let mut updates: Vec<Vec<u32>> = Vec::new();

    for update in updates_part {
        let update_parts = update.split(",").collect::<Vec<&str>>();
        let mut update_vec: Vec<u32> = Vec::new();
        for part in update_parts {
            update_vec.push(part.parse::<u32>().unwrap());
        }
        updates.push(update_vec);
    }

    for update in updates {
        let mut passed = true;

        for rule in rules.iter() {
            let(x_maybe_idx, y_maybe_idx) = (update.iter().position(|&x| x == rule.0), update.iter().position(|&x| x == rule.1));
            if x_maybe_idx.is_none() || y_maybe_idx.is_none() {
                continue
            }

            let (x_idx, y_idx) = (x_maybe_idx.unwrap(), y_maybe_idx.unwrap());
            if x_idx > y_idx { // x must be before y
                passed = false;

                break;
            }
        }

        if passed {
            sum += update.get(update.len() / 2).unwrap()
        }
    }


    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
        assert_eq!(result, "143");
    }
}
