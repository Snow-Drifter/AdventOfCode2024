use crate::input_util;
use crate::solutions::Solution;

pub struct Day09;

impl Solution for Day09 {
    fn part_one(input_dir: &str) -> i64 {
        let lines: Vec<_> = input_util::read_file_buffered(&input_dir)
            .map(|x| x.unwrap())
            .map(|x| {
                x.chars()
                    .map(|char| char.to_digit(10).expect("digit"))
                    .collect()
            })
            .next()
            .unwrap();
        Self::actual_one(lines)
    }

    fn part_two(input_dir: &str) -> i64 {
        let lines: Vec<_> = input_util::read_file_buffered(&input_dir)
            .map(|x| x.unwrap())
            .map(|x| {
                x.chars()
                    .map(|char| char.to_digit(10).expect("digit"))
                    .collect()
            })
            .next()
            .unwrap();
        Self::actual_two(lines)
    }
}

impl Day09 {
    fn actual_one(input: Vec<u32>) -> i64 {
        let mut idigits = input.iter();
        let first_block = *idigits.next().unwrap();
        let mut expanded = vec![];
        for data in 0..first_block {
            expanded.push(0);
        }
        let mut id = 1;
        loop {
            if let Some(space) = idigits.next() {
                expanded.append(&mut vec![u32::MAX].repeat(*space as usize));
            } else {
                break;
            }
            if let Some(block) = idigits.next() {
                expanded.append(&mut vec![id].repeat(*block as usize));
                id += 1;
            } else {
                break;
            }
        }
        println!("{:?}", expanded);
        let reverse_allocation: Vec<_> =
            expanded.iter().rev().filter(|&x| u32::MAX.ne(x)).collect();
        println!("{:?}", reverse_allocation);
        let mut ireverse = reverse_allocation.iter();
        let mut output = vec![];
        let end_point = reverse_allocation.len();
        let mut index = 0;
        for data in expanded.iter() {
            if index >= end_point {
                break;
            }
            index += 1;
            if u32::MAX.eq(&data) {
                if let Some(&next) = ireverse.next() {
                    output.push(next);
                } else {
                    break;
                }
            } else {
                output.push(data);
            }
        }
        // println!("{:?}", output);
        output
            .iter()
            .enumerate()
            .map(|(index, id)| index as u128 * **id as u128)
            .sum::<u128>() as i64
    }

    fn actual_two(input: Vec<u32>) -> i64 {
        let mut index = 0;
        let mut id = 0;
        let mut data = true;
        let mut tokens: Vec<Data> = input.iter().map(|size| {
            let size = *size as usize;
            let token = if data {
                id += 1;
                Data::File(index, size, id - 1)
            } else {
                Data::EmptyBlock(index, size)
            };
            index += size;
            token
        }).collect();
        // let mut moved_ids = vec![];
        let files_to_rearrange: Vec<_> = tokens.iter().filter(|x| matches!(x, Data::File(_, _, _))).collect();
        for file in files_to_rearrange.iter() {
            if let Data::File(index, size, id) = file {
                // tokens = tokens.iter()
            }
        }
        0
    }
}

enum Data {
    File(usize, usize, usize),
    EmptyBlock(usize, usize),
}

impl Data {
    fn checksum(&self) -> usize {
        if let Data::File(index, size, id) = self {
            (*index..*index + *size).map(|index| index * id).sum()
        } else {
            0
        }
    }
}

#[test]
fn given() {
    let input = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
    let answer = Day09::actual_one(input);
    assert_eq!(answer, 1928);
}
