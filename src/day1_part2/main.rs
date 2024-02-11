use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use num_format::{Locale, ToFormattedString};

fn main() {
    //Read data
    let path = Path::new("src/day1_part1/input.txt");
    let file = File::open(&path).unwrap();
    let lines_iter = BufReader::new(file)
        .lines()
        .flatten();

    //Parse and add calibration values to container
    let mut calibration_values = Vec::<u32>::new();
    for line in lines_iter {
        let mut parsed_digits = Vec::<u32>::new();
        let mut cursor_index = 0usize;

        while cursor_index < line.len().try_into().unwrap() {
            let char = line.chars().nth(cursor_index).unwrap();
            let char_parse_result = char.to_digit(10);

            if char_parse_result.is_some() {
                parsed_digits.push(char_parse_result.unwrap());
            } else {
                try_parse_digit_string(cursor_index, &line, &mut parsed_digits);
            }

            cursor_index += 1
        }

        calibration_values.push(10 * parsed_digits.first().unwrap() + parsed_digits.last().unwrap());
    }

    //Sum results and print final answer
    println!("Sum = {}", calibration_values.iter().sum::<u32>().to_formatted_string(&Locale::en));
}

fn try_parse_digit_string(cursor_index: usize, line: &str, parsed_digits: &mut Vec::<u32>) {
    const MAX_BUFFER_LENGTH : usize = 5;
    let mut digit_char_buffer = Vec::<char>::new();
    let mut break_early = false;
    let mut relative_cursor_index = 0usize;

    while relative_cursor_index < MAX_BUFFER_LENGTH &&
        cursor_index + relative_cursor_index <= line.len() - 1 &&
        !break_early {

        let char = line.chars().nth((cursor_index + relative_cursor_index).try_into().unwrap()).unwrap();
        if char.to_digit(10).is_some() {
            break_early = true;
        } else {
            digit_char_buffer.push(char);

            match parse_digit_char_buffer(&digit_char_buffer.iter().collect::<String>()) {
                Ok(digit) => {
                    parsed_digits.push(digit);
                    break_early = true;
                },
                Err(_e) => {}
            }
        }

        relative_cursor_index += 1;
    }
}

fn parse_digit_char_buffer(input: &str) -> Result<u32, &'static str> {
    return match input {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => Err("Not a valid digit string")
    };
}