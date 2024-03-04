use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_format::{Locale, ToFormattedString};

fn main() {
    //Parse data
    let path = Path::new("src/day15_part1/input.txt");
    let data = parse_data(&path);

    //Calculate and print answer
    let hash_sum = data
        .split(",")
        .map(|x| calculate_step_hash(x))
        .sum::<u64>();
    println!("The hash sum is {}", hash_sum.to_formatted_string(&Locale::en))
}

//Single line file so casting vector to scalar
fn parse_data(path: &Path) -> String {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()[0].clone();
}

fn calculate_step_hash(step: &str) -> u64 {
    const INITIAL_VALUE: u64 = 0;
    const MULTIPLIER: u64 = 17;
    const MODULUS: u64 = 256;

    return step
        .chars()
        .fold(INITIAL_VALUE, |acc, x| {
            let mut temp = acc + x as u64;
            temp *= MULTIPLIER;
            return temp % MODULUS;
        });
}