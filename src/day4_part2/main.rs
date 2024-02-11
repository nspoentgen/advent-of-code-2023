use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;
use std::convert::TryInto;
use num_format::{Locale, ToFormattedString};


fn main() {
    //Read file, parse data, and compute game scores
    let path = Path::new("src/day4_part1/input.txt");
    let file = File::open(&path).unwrap();
    let game_scores = BufReader::new(file)
        .lines()
        .map(|l| parse_game_results(&l.unwrap()))
        .collect::<Vec<u32>>();

    //Calculate and print the number of cards
    println!("The total number of cards is {}", compute_num_cards(&game_scores).to_formatted_string(&Locale::en))
}

fn parse_game_results(line: &String) -> u32 {
    const HEADER_DATA_SEPARATOR: char = ':';
    const WINNING_TEST_NUMBERS_SEPARATOR: char = '|';
    const NUMBERS_SEPARATOR: char = ' ';

    //Get game data string
    let game_data_str = line.split(HEADER_DATA_SEPARATOR).collect::<Vec<&str>>()[1];

    //Split string into winning numbers and test numbers
    let game_data_str_parts = game_data_str.split(WINNING_TEST_NUMBERS_SEPARATOR).collect::<Vec<&str>>();
    let winning_numbers_str = game_data_str_parts[0].trim();
    let test_numbers_str = game_data_str_parts[1].trim();

    //Parse strings
    let winning_numbers = winning_numbers_str
        .split(NUMBERS_SEPARATOR)
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let test_numbers = test_numbers_str
        .split(NUMBERS_SEPARATOR)
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return compute_game_score(&(winning_numbers, test_numbers));
}

fn compute_game_score(game_data: &(Vec<u32>, Vec<u32>)) -> u32 {
    let winning_set = HashSet::<&u32>::from_iter(&game_data.0);
    return game_data.1
        .iter()
        .filter(|test_num| winning_set.contains(test_num))
        .count() as u32;
}

fn compute_num_cards(game_scores: &Vec<u32>) -> u32 {
    let mut num_cards = vec![1usize; game_scores.len()].into_boxed_slice();

    for num_cards_index in 0..num_cards.len() {
        for _ in 0..num_cards[num_cards_index] {
            for relative_index in 1usize..=game_scores[num_cards_index].try_into().unwrap() {
                let bonus_index = num_cards_index + relative_index;
                if bonus_index <= num_cards.len() - 1 {
                    num_cards[bonus_index] += 1;
                }
            }
        }
    }

    return num_cards.iter().sum::<usize>() as u32;
}