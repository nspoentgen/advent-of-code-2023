use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::collections::HashMap;
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
    /*
        let mut cache = HashMap::<Vec<Vec<char>>, usize>::new();

   match cache.get(&data) {
       Some(value) => {
           println!("Cycle {} repeat of cycle {}", cycle, value);
       },
       None => {
           cache.insert(data.clone(), cycle);
       }
   };
     */
    //0-indexing
    //92-163 inclusive cycle

    let mut cache = HashMap::<usize, Vec<Vec<char>>>::new();
    for cycle in 0usize..=163 {
        shift_north(&mut data);
        shift_west(&mut data);
        shift_south(&mut data);
        shift_east(&mut data);

        if cycle >= 92 {
            cache.insert(cycle - 92, data.clone());
        }
    }

    let cache_cycle = (1000000000usize - 1 - 92) % 72;
    let final_data = &cache[&cache_cycle];

    //print_debug(&data);


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

fn print_debug(data: &Vec<Vec<char>>) {
for row in data.iter() {
   for element in row.iter() {
       print!("{}", element);
   }
   print!("\n");
}
}

fn shift_north(data: &mut Vec<Vec<char>>) {
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

fn shift_west(data: &mut Vec<Vec<char>>) {
let mut rock_shifted = true;

while (rock_shifted) {
   rock_shifted = false;

   //Rectangular input so using first row arbitrarily for max column index
   for (row_index, col_index) in (0..data.len()).cartesian_product(1..data[0].len()) {
       if data[row_index][col_index] == MOVABLE && data[row_index][col_index - 1] == EMPTY {
           data[row_index][col_index] = EMPTY;
           data[row_index][col_index - 1] = MOVABLE;
           rock_shifted = true;
       }
   }
}
}

fn shift_south(data: &mut Vec<Vec<char>>) {
let mut rock_shifted = true;

while (rock_shifted) {
   rock_shifted = false;

   //Rectangular input so using first row arbitrarily for max column index
   for (row_index, col_index) in (0..data.len() - 1).cartesian_product(0..data[0].len()) {
       if data[row_index][col_index] == MOVABLE && data[row_index + 1][col_index] == EMPTY {
           data[row_index][col_index] = EMPTY;
           data[row_index + 1][col_index] = MOVABLE;
           rock_shifted = true;
       }
   }
}
}

fn shift_east(data: &mut Vec<Vec<char>>) {
let mut rock_shifted = true;

while (rock_shifted) {
   rock_shifted = false;

   //Rectangular input so using first row arbitrarily for max column index
   for (row_index, col_index) in (0..data.len()).cartesian_product(0..data[0].len() - 1) {
       if data[row_index][col_index] == MOVABLE && data[row_index][col_index + 1] == EMPTY {
           data[row_index][col_index] = EMPTY;
           data[row_index][col_index + 1] = MOVABLE;
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