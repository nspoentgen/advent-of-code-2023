use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

#[macro_use]
extern crate timeit;

const FIXED: char = '#';
const MOVABLE: char = 'O';
const EMPTY: char = '.';

fn main() {
    //Parse data
    let path = Path::new("src/day14_part1/input.txt");
    let mut data = parse_data(&path);

    timeit!({
        shift_rocks(&mut data);
    });

    //Calculate load and print answer
    let total_rock_load = (0..data.len())
        .cartesian_product(0..data[0].len())
        .into_iter()
        .map(|(row_index, col_index)| calculate_rock_load(row_index, col_index, &data))
        .sum::<usize>();
    println!("The total load is {}", total_rock_load.to_formatted_string(&Locale::en))
}

fn parse_data(path: &Path) -> Vec<Vec<char>> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
}

fn shift_rocks(data: &mut Vec<Vec<char>>) {
    let mut rock_shifted = true;

    while (rock_shifted) {
        rock_shifted = false;

        //Rectangular input so using first row arbitrarily for max column index
        for (row_index, col_index) in (1..data.len()).cartesian_product(0..data[0].len()) {
            if data[row_index][col_index] == MOVABLE && data[row_index - 1][col_index] == EMPTY {
                data[row_index][col_index] = EMPTY;
                data[row_index - 1][col_index] = MOVABLE;
                rock_shifted = true;
            }
        }
    }
}

fn calculate_rock_load(row_index: usize, col_index: usize, data: &Vec<Vec<char>>) -> usize {
    return match data[row_index][col_index] {
        MOVABLE => data.len() - row_index,
        _ => 0
    };
}