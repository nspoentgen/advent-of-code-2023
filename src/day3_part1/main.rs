use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use itertools::iproduct;
use itertools::izip;

#[repr(u8)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum SchematicResult {
    Noise = 0,
    Digit = 1,
    Symbol = 2
}

fn main() {
    //Read data
    let path = Path::new("src/day3_part1/input.txt");
    let file = File::open(&path).unwrap();
    let raw_data = BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();

    //Map data to more useful form
    let processed_data = map_data_to_enum(&raw_data);

    //Find valid digits
    let valid_digits = flag_valid_digits(&processed_data);

    //Convert digits to valid number
    let valid_numbers = extract_valid_numbers(&raw_data, &processed_data, &valid_digits);

    //Print final result
    println!("The sum of the engine parts is {}", valid_numbers.iter().sum::<u32>().to_formatted_string(&Locale::en))
}

fn map_data_to_enum(raw_data: &Vec<String>) -> Vec<Vec<SchematicResult>> {
    let mut processed_data = Vec::<Vec<SchematicResult>>::new();

    for raw_line in raw_data {
        let mut processsed_line = Vec::<SchematicResult>::new();

        for char in raw_line.chars() {
            let element_result: SchematicResult;

            if char == '.' {
                element_result = SchematicResult::Noise;
            } else if char.is_digit(10) {
                element_result = SchematicResult::Digit;
            } else {
                element_result = SchematicResult::Symbol;
            }

            processsed_line.push(element_result);
        }

        processed_data.push(processsed_line);
    }

    return processed_data;
}

fn flag_valid_digits(processed_data: &Vec<Vec<SchematicResult>>) -> Vec<Vec<bool>> {
    let mut valid_digits = Vec::<Vec<bool>>::new();

    for row_index in 0..processed_data.len() {
        let mut valid_digits_row = Vec::<bool>::new();

        for col_index in 0..processed_data[row_index].len() {
            valid_digits_row.push(processed_data[row_index][col_index] == SchematicResult::Digit &&
                has_adjacent_symbol(&processed_data, (row_index, col_index)));
        }

        valid_digits.push(valid_digits_row);
    }

    return valid_digits;
}

fn has_adjacent_symbol(processed_data: &Vec<Vec<SchematicResult>>, test_coord: (usize, usize)) -> bool {
    const RELATIVE_1D_POS: [i32; 3] = [-1, 0, 1];

    let adjacent_to_symbol = |relative_coord: (&i32, &i32)| -> bool {
        let row_index: usize = ((test_coord.0 as i32) + *relative_coord.0) as usize;
        let col_index: usize = ((test_coord.1 as i32) + *relative_coord.1) as usize;

        return if (*relative_coord.0 == 0 && *relative_coord.1 == 0) ||
            row_index > processed_data.len() - 1 ||
            col_index > processed_data[0].len() - 1 {
            false
        } else {
            processed_data[row_index][col_index] == SchematicResult::Symbol
        }
    };

    return iproduct!(RELATIVE_1D_POS.iter(), RELATIVE_1D_POS.iter())
        .any(adjacent_to_symbol);
}


fn extract_valid_numbers(raw_data: &Vec<String>, processed_data: &Vec<Vec<SchematicResult>>, valid_digits: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut valid_numbers = Vec::<u32>::new();

    for i in 0..raw_data.len() {
        let mut char_buffer = Vec::<char>::new();
        let mut valid_digit_buffer = Vec::<bool>::new();

        for item_summary in izip!(raw_data[i].chars(), processed_data[i].iter(), valid_digits[i].iter()) {
            if *item_summary.1 == SchematicResult::Digit {
                char_buffer.push(item_summary.0);
                valid_digit_buffer.push(*item_summary.2);
            } else {
                if char_buffer.len() > 0 && valid_digit_buffer.iter().any(|x| *x) {
                    valid_numbers.push(char_buffer.iter().collect::<String>().parse::<u32>().unwrap())
                }

                char_buffer.clear();
                valid_digit_buffer.clear();
            }
        }

        //Need to check after end in case final item is a digit
        if char_buffer.len() > 0 && valid_digit_buffer.iter().any(|x| *x) {
            valid_numbers.push(char_buffer.iter().collect::<String>().parse::<u32>().unwrap())
        }
    }

    return valid_numbers;
}
