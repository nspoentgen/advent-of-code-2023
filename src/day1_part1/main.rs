use io::BufReader;
use std::fs::File;
use std::io;
use std::io::BufRead;
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
        let mut num1 : Option<u32> = None;
        let mut num2 : Option<u32> = None;

        for char in line.chars() {
            let result = char.to_digit(10);
            if result.is_some() {
                num1 = Some(result.unwrap());
                break;
            }
        }

        for char in line.chars().rev() {
            let result = char.to_digit(10);
            if result.is_some() {
                num2 = Some(result.unwrap());
                break;
            }
        }

       if num1.is_none() || num2.is_none() {
           panic!("Something is wrong. Could not parse the digits.");
       }

       calibration_values.push(10 * num1.unwrap() + num2.unwrap());
   }

    println!("Sum = {}", calibration_values.iter().sum::<u32>().to_formatted_string(&Locale::en));
}
