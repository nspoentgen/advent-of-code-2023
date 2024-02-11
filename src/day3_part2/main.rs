use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};


fn main() {
    //Read data
    let path = Path::new("src/day3_part1/input.txt");
    let file = File::open(&path).unwrap();
    let raw_data = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();

    //Get product of gear numbers for gears that have exactly two gear numbers
    const NUM_REQUIRED_GEAR_NUMBERS: usize = 2;
    let product_sum = get_gear_numbers(&raw_data)
        .iter()
        .filter(|x| x.len() == NUM_REQUIRED_GEAR_NUMBERS)
        .map(|x| x[0]*x[1])
        .sum::<u32>();

    //Print result
    println!("The product sum of the valid gears is {}", product_sum.to_formatted_string(&Locale::en))
}

//Get valid gear numbers for every gear symbol
fn get_gear_numbers(raw_data: &Vec<String>) -> Vec<Vec<u32>> {
    const GEAR_SYMBOL: char = '*';
    let mut all_gear_numbers = Vec::<Vec<u32>>::new();

    for row_index in 0..raw_data.len() {
        for col_index in 0..raw_data[row_index].len() {
            if raw_data[row_index].chars().nth(col_index).unwrap() == GEAR_SYMBOL {
                let mut local_gear_numbers = Vec::<u32>::new();

                //Get any adjacent cells that have digits then use that info to extract the actual number
                for digit_pos in get_adjacent_digit_cell_positions(&raw_data, (row_index, col_index)) {
                    if digit_pos.1 < col_index {
                        local_gear_numbers.push(resolve_number(&raw_data[digit_pos.0], digit_pos.1));
                    } else if digit_pos.1 >= col_index && digit_pos != (row_index, col_index) {
                        let left_neighbor_is_digit = digit_pos.0 <= raw_data.len() - 1 &&
                            digit_pos.1 - 1  <= raw_data[digit_pos.0].len() - 1 &&
                            raw_data[digit_pos.0].chars().nth(digit_pos.1 - 1).unwrap().is_digit(10);

                        if !left_neighbor_is_digit {
                            local_gear_numbers.push(resolve_number(&raw_data[digit_pos.0], digit_pos.1));
                        }
                    }
                }

                all_gear_numbers.push(local_gear_numbers);
            }
        }
    }

    return all_gear_numbers;
}

//Flags any adjacent cell from the current position that contains a digit. Note the digit can be at any position
//in the gear number. The caller will have to determine how to extract the actual number given this function's information.
fn get_adjacent_digit_cell_positions(raw_data: &Vec<String>, gear_pos: (usize, usize)) -> Vec<(usize, usize)> {
    const RELATIVE_1D_POS: [i32; 3] = [-1, 0, 1];
    let mut gear_number_starting_positions = Vec::<(usize, usize)>::new();

    for relative_row_pos in RELATIVE_1D_POS {
        for relative_col_pos in RELATIVE_1D_POS {
            let row_index: usize = ((gear_pos.0 as i32) + relative_row_pos) as usize;
            let col_index: usize = ((gear_pos.1 as i32) + relative_col_pos) as usize;

            //Handles moving left and right cases because usize is always >= 0 and oveflowing will wrap to
            //large usize value which wil always be >>> than col size for this case
            if row_index <= raw_data.len() - 1 &&
               col_index <= raw_data[row_index].len() - 1 &&
               raw_data[row_index].chars().nth(col_index).unwrap().is_digit(10) {
                gear_number_starting_positions.push((row_index, col_index));
            }
        }
    }

    return gear_number_starting_positions;
}

//Assumes starting position is a number
fn resolve_number(row_data: &String, mut col_index: usize) -> u32 {
    let mut digit_buffer = Vec::<char>::new();
    let mut end_of_number = false;

    //Rewind to start of number. Handles moving left and right cases because usize is always >= 0 and oveflowing will wrap to
    //large usize value which wil always be >>> than col size for this case
    while !end_of_number {
        end_of_number = col_index > row_data.len() - 1 || !row_data.chars().nth(col_index).unwrap().is_digit(10);
        col_index = col_index.wrapping_sub(1);
    }
    col_index = col_index.wrapping_add(2);

    //Grab and parse number
    while col_index <= row_data.len() - 1 && row_data.chars().nth(col_index).unwrap().is_digit(10) {
        digit_buffer.push(row_data.chars().nth(col_index).unwrap());
        col_index += 1;
    }

    return digit_buffer.iter().collect::<String>().parse::<u32>().unwrap();
}