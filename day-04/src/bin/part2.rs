#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);

    dbg!(output);
}

fn part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut count = 0;
    // Define all possible X-MAS patterns with 'M's and 'S's in different positions
    let patterns = vec![
        // Original pattern
        [('M', -1, -1), ('A', 0, 0), ('S', -1, 1), ('M', 1, -1), ('S', 1, 1)],
        // Rotated 90 degrees
        [('M', -1, 1), ('A', 0, 0), ('S', 1, 1), ('M', -1, -1), ('S', 1, -1)],
        // Rotated 180 degrees
        [('M', 1, 1), ('A', 0, 0), ('S', 1, -1), ('M', -1, 1), ('S', -1, -1)],
        // Rotated 270 degrees
        [('M', 1, -1), ('A', 0, 0), ('S', -1, -1), ('M', 1, 1), ('S', -1, 1)],
    ];

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            for pattern in &patterns {
                let mut match_pattern = true;
                for &(expected_char, dx, dy) in pattern {
                    let x = i as isize + dx;
                    let y = j as isize + dy;
                    if x < 0 || y < 0 || x >= rows as isize || y >= cols as isize {
                        match_pattern = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != expected_char {
                        match_pattern = false;
                        break;
                    }
                }
                if match_pattern {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(result, "9");
    }
}
