use crate::input_util;
use crate::solutions::Solution;

pub struct Day04;

impl Solution for Day04 {
    fn part_one(input_dir: &str) -> i64 {
        let lines: Vec<Vec<_>> = input_util::read_file_buffered(&input_dir)
            .map(|x| x.unwrap())
            .map(|x| x.chars().collect())
            .collect();
        // let window_size = 4;
        // let mut count = 0;
        // for col in 0..=lines.len() - window_size {
        //     for row in 0..=lines[0].len() - window_size {
        //         let window: Vec<Vec<char>> = (row..row + window_size)
        //             .map(|r| lines[r][col..col + window_size].to_vec())
        //             .collect();
        //         // count += Self::count_xmas(&window);
        //         count += Self::chat_gpt(&window, "XMAS");
        //     }
        // }
        // count as i64
        Self::chat_gpt(&lines, "XMAS") as i64
    }

    fn part_two(input_dir: &str) -> i64 {
        0
    }
}

impl Day04 {
    fn count_xmas(lines: &[Vec<char>]) -> usize {
        // println!("{:?}", lines);
        let xmas = vec!['X', 'M', 'A', 'S'];
        let mut reverse_xmas = xmas.clone();
        reverse_xmas.reverse();

        let mut count = 0;
        for line in lines {
            if xmas.eq(line) || reverse_xmas.eq(line) {
                count += 1;
            }
        }
        if (0..4).all(|i| xmas[i] == lines[i][i]) {
            count += 1;
        }
        if (0..4).all(|i| reverse_xmas[i] == lines[i][i]) {
            count += 1;
        }
        if (0..4).all(|i| xmas[i] == lines[i][3 - i]) {
            count += 1;
        }
        if (0..4).all(|i| reverse_xmas[i] == lines[i][3 - i]) {
            count += 1;
        }
        for x in 0..4 {
            if (0..4).all(|y| lines[y][x] == xmas[y]) {
                count += 1;
            }
            if (0..4).all(|y| lines[y][x] == reverse_xmas[y]) {
                count += 1;
            }
        }
        count
    }

    fn chat_gpt(grid: &Vec<Vec<char>>, word: &str) -> usize {
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        let word_len = word.len();
        let reversed_word: String = word.chars().rev().collect(); // Reverse the word

        // Check horizontal (left-to-right) and (right-to-left) matches
        for r in 0..rows {
            for c in 0..=(cols - word_len) {
                let slice: String = grid[r][c..c + word_len].iter().collect();
                if slice == word || slice == reversed_word {
                    count += 1;
                }
            }
        }

        // Check vertical (top-to-bottom) and (bottom-to-top) matches
        for c in 0..cols {
            for r in 0..=(rows - word_len) {
                let mut match_found = true;
                let mut reversed_match_found = true;
                for i in 0..word_len {
                    if grid[r + i][c] != word.chars().nth(i).unwrap() {
                        match_found = false;
                    }
                    if grid[r + i][c] != reversed_word.chars().nth(i).unwrap() {
                        reversed_match_found = false;
                    }
                }
                if match_found || reversed_match_found {
                    count += 1;
                }
            }
        }

        // Check diagonal (top-left to bottom-right) and (bottom-right to top-left) matches
        for r in 0..=(rows - word_len) {
            for c in 0..=(cols - word_len) {
                let mut match_found = true;
                let mut reversed_match_found = true;
                for i in 0..word_len {
                    if grid[r + i][c + i] != word.chars().nth(i).unwrap() {
                        match_found = false;
                    }
                    if grid[r + i][c + i] != reversed_word.chars().nth(i).unwrap() {
                        reversed_match_found = false;
                    }
                }
                if match_found || reversed_match_found {
                    count += 1;
                }
            }
        }

        // Check diagonal (top-right to bottom-left) and (bottom-left to top-right) matches
        for r in 0..=(rows - word_len) {
            for c in (word_len - 1)..cols {
                let mut match_found = true;
                let mut reversed_match_found = true;
                for i in 0..word_len {
                    if grid[r + i][c - i] != word.chars().nth(i).unwrap() {
                        match_found = false;
                    }
                    if grid[r + i][c - i] != reversed_word.chars().nth(i).unwrap() {
                        reversed_match_found = false;
                    }
                }
                if match_found || reversed_match_found {
                    count += 1;
                }
            }
        }

        count
    }

}
