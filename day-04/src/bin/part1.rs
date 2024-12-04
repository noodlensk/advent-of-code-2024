#![warn(clippy::pedantic)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);

    dbg!(output);
}

fn part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let word = "XMAS";
    let word_len = word.len();

    // All 8 possible directions (dx, dy)
    let directions = vec![
        (-1, -1), // Up-Left
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, -1),  // Left
        (0, 1),   // Right
        (1, -1),  // Down-Left
        (1, 0),   // Down
        (1, 1),   // Down-Right
    ];

    let mut count = 0;
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(&grid, word, x as isize, y as isize, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}
fn check_word(
    grid: &Vec<Vec<char>>,
    word: &str,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
) -> bool {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let rows = grid.len() as isize;
    let cols = if rows > 0 { grid[0].len() as isize } else { 0 };

    for i in 0..word_len {
        let nx = x + dx * i as isize;
        let ny = y + dy * i as isize;

        if nx < 0 || nx >= rows || ny < 0 || ny >= cols {
            return false;
        }

        if grid[nx as usize][ny as usize] != word_chars[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(result, "18");
    }
}
