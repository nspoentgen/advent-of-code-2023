use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::iter::zip;
use num_format::{Locale, ToFormattedString};

fn main () {
    //Parse data
    let path = Path::new("src/day6_part1/input.txt");
    let input_data = parse_data(&path);

    //Calculate product of num ways to win each game
    let game_win_product = input_data
        .iter()
        .map(|x| get_winning_game_count(x.0, x.1))
        .product::<u64>();

    //Print the final result
    println!("The winning game count product is {}", game_win_product.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<(u64, u64)> {
    let file = File::open(&path).unwrap();
    let mut line_iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    let times = line_iter
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = line_iter
        .next()
        .unwrap()
        .split(":")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    return zip(times, distances)
        .collect::<Vec<(u64, u64)>>()
}

fn get_winning_game_count(max_time: u64, record_distance: u64) -> u64 {
    return (1..=max_time-1)
        .into_iter()
        .map(|x| x * (max_time-x))
        .filter(|x| *x > record_distance)
        .count() as u64;
}