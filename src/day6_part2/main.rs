use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};

fn main () {
    //Parse data
    let path = Path::new("src/day6_part1/input.txt");
    let input_data = parse_data(&path);

    //Print the final result
    println!("The winning game count product is {}",
             get_winning_game_count(input_data.0, input_data.1).to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> (u64, u64) {
    let file = File::open(&path).unwrap();
    let mut line_iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    let time = line_iter
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = line_iter
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    return (time, distance)
}

fn get_winning_game_count(max_time: u64, record_distance: u64) -> u64 {
    return (1..=max_time-1)
        .into_iter()
        .map(|x| x * (max_time-x))
        .filter(|x| *x > record_distance)
        .count() as u64;
}