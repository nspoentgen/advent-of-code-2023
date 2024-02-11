use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};

use crate::hand::Hand;
use crate::hand::Card;

mod hand;

fn main() {
    //Parse data
    let path = Path::new("src/day7_part1/input.txt");
    let mut input = parse_data(&path);

    //Calculate final result
    input.sort();
    let total_score = input
        .iter()
        .enumerate()
        .map(|x| ((x.0 as u32 + 1) * x.1.get_bid()) as u64)
        .sum::<u64>();
    println!("The total score is {}", total_score.to_formatted_string(&Locale::en))
}

fn parse_data(path: &Path) -> Vec<Hand> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter()
        .map(|l| parse_hand_data(&l))
        .collect::<Vec<Hand>>();
}

fn parse_hand_data(line: &String) -> Hand {
    let hand_parts = line.split(" ").collect::<Vec<&str>>();
    let cards_raw = hand_parts[0];
    let bid = hand_parts[1].parse::<u32>().unwrap();

    let mut cards: [Card; 5] = Default::default();
    for index in 0..cards.len() {
        match cards_raw.chars().nth(index).unwrap() {
            '2' => cards[index] = Card::Two,
            '3' => cards[index] = Card::Three,
            '4' => cards[index] = Card::Four,
            '5' => cards[index] = Card::Five,
            '6' => cards[index] = Card::Six,
            '7' => cards[index] = Card::Seven,
            '8' => cards[index] = Card::Eight,
            '9' => cards[index] = Card::Nine,
            'T' => cards[index] = Card::Ten,
            'J' => cards[index] = Card::Jack,
            'Q' => cards[index] = Card::Queen,
            'K' => cards[index] = Card::King,
            'A' => cards[index] = Card::Ace,
            _ => panic!("Could not map card char")
        }
    }

    return Hand::new(cards, bid);
}