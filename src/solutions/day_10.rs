use crate::input_util;
use crate::solutions::Solution;
use std::collections::HashSet;

pub struct Day10;

impl Solution for Day10 {
    fn part_one(dir: &str) -> i64 {
        let cells = Self::parse(dir);
        cells
            .iter()
            .filter(|cell| cell.height == 0)
            .map(|cell| cell.start_to_finish(&cells))
            .map(|finishes| finishes.iter().collect::<HashSet<_>>().len())
            .sum::<usize>() as i64
    }

    fn part_two(dir: &str) -> i64 {
        let cells = Self::parse(dir);
        cells
            .iter()
            .filter(|cell| cell.height == 0)
            .flat_map(|cell| cell.start_to_finish(&cells))
            .count() as i64
    }
}

impl Day10 {
    fn parse(dir: &str) -> Vec<Cell> {
        input_util::read_file_buffered(dir)
            .map(|maybe_line| maybe_line.unwrap())
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, &height)| Cell { x, y, height })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    height: u32,
}

impl Cell {
    fn start_to_finish(&self, all: &[Cell]) -> Vec<Cell> {
        let mut pointers = vec![*self];
        for height in 1..10 {
            pointers = pointers
                .into_iter()
                .flat_map(|cell| cell.neighbors(all))
                .filter(|cell| cell.height == height)
                .collect();
        }
        pointers
    }
    fn neighbors(&self, all: &[Cell]) -> Vec<Cell> {
        all.into_iter()
            .filter(|cell| self.is_neighbor(cell))
            .copied()
            .collect()
    }

    fn is_neighbor(&self, other: &Cell) -> bool {
        // north
        if self.x == other.x && self.y == other.y + 1 {
            true
        }
        // east
        else if self.x + 1 == other.x && self.y == other.y {
            true
        }
        // south
        else if self.x == other.x && self.y + 1 == other.y {
            true
        }
        // west
        else if self.x == other.x + 1 && self.y == other.y {
            true
        } else {
            false
        }
    }
}
