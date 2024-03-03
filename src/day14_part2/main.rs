use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;
use core::ops::Range;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

const FIXED: char = '#';
const MOVABLE: char = 'O';
const EMPTY: char = '.';

enum ShiftDirection {
    NORTH,
    WEST,
    SOUTH,
    EAST
}

fn main() {
    //Constants
    const NUM_CYCLES: usize = 1000000000;
    const LIMIT_CYCLE_OFFSET: usize = 92;
    const LIMIT_CYCLE_LENGTH: usize = 72;

    //Parse data
    let path = Path::new("src/day14_part1/input.txt");
    let mut data = parse_data(&path);

    //We can't shift for 1 billion cycles, so we need to find a shortcut. Looking at the data,
    //we see there is a limit cycle in the rock data. The limit cycle has an offset of 92 and
    //a length of 72 (using 0-based indexing). Cache all the possiblilties using the
    //module limit cycle number for the cycle
    let mut cache = HashMap::<usize, Vec<Vec<char>>>::new();
    for cycle in 0usize..(LIMIT_CYCLE_OFFSET + LIMIT_CYCLE_LENGTH) {
        shift_rocks(&mut data, ShiftDirection::NORTH);
        shift_rocks(&mut data, ShiftDirection::WEST);
        shift_rocks(&mut data, ShiftDirection::SOUTH);
        shift_rocks(&mut data, ShiftDirection::EAST);

        if cycle >= LIMIT_CYCLE_OFFSET {
            cache.insert(cycle - LIMIT_CYCLE_OFFSET, data.clone());
        }
    }

    //Predict with the final rock layout using the logic outlined above
    let cache_cycle = (NUM_CYCLES - 1 - LIMIT_CYCLE_OFFSET) % LIMIT_CYCLE_LENGTH;
    let final_data = &cache[&cache_cycle];

    //Calculate load and print answer
    let total_rock_load = (0..final_data.len())
       .cartesian_product(0..final_data[0].len())
       .into_iter()
       .map(|(row_index, col_index)| calculate_rock_load(row_index, col_index, &final_data))
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

fn shift_rocks(data: &mut Vec<Vec<char>>, shift_direction: ShiftDirection) {
    let (row_range, col_range, row_shift_offset, col_shift_offset): (Range<isize>, Range<isize>, isize, isize) = match shift_direction {
        ShiftDirection::NORTH => (1..data.len() as isize, 0..data[0].len() as isize, -1, 0),
        ShiftDirection::WEST => (0..data.len() as isize, 1..data[0].len() as isize, 0, -1),
        ShiftDirection::SOUTH => (0..data.len() as isize - 1, 0..data[0].len() as isize, 1, 0),
        ShiftDirection::EAST => (0..data.len() as isize, 0..data[0].len() as isize - 1, 0, 1)
    };

    let mut rock_shifted = true;
    while rock_shifted {
        rock_shifted = false;

        //Rectangular input so using first row arbitrarily for max column index
        for (row_index, col_index) in row_range.clone().cartesian_product(col_range.clone()) {
            if data[row_index as usize][col_index as usize] == MOVABLE && data[(row_index + row_shift_offset) as usize][(col_index + col_shift_offset) as usize] == EMPTY {
                data[row_index as usize][col_index as usize] = EMPTY;
                data[(row_index + row_shift_offset) as usize][(col_index + col_shift_offset) as usize] = MOVABLE;
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