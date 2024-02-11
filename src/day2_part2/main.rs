use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::iter::Flatten;
use std::io::Lines;
use std::collections::HashMap;
use regex::Regex;
use num_format::{Locale, ToFormattedString};

fn main() {
    //Read data
    let path = Path::new("src/day2_part1/input.txt");
    let file = File::open(&path).unwrap();
    let lines_iter = BufReader::new(file)
        .lines()
        .flatten();

    //Parse input
    let all_games_summary = parse_results(lines_iter);
    //println!("{:?}", all_games_summary);

    //Calculate result
    let get_max_powers = |game_result: &Vec<(u32,u32,u32)>| -> (u32, u32, u32) {
        let mut max_r = 0u32;
        let mut max_g = 0u32;
        let mut max_b = 0u32;

        for round_values in game_result {
            if round_values.0 > max_r {
                max_r = round_values.0
            }

            if round_values.1 > max_g {
                max_g = round_values.1
            }

            if round_values.2 > max_b {
                max_b = round_values.2
            }
        }

        return (max_r, max_g, max_b);
    };

    let sum_powers = all_games_summary
        .iter()
        .map(|(_,v)| get_max_powers(v))
        .map(|x| x.0 * x.1 * x.2)
        .sum::<u32>();

    //Print result
    println!("The sum of max powers is {}", sum_powers.to_formatted_string(&Locale::en));
}

fn parse_results(all_results_iter: Flatten<Lines<BufReader<File>>>) -> HashMap::<u32, Vec<(u32, u32, u32)>> {
    //Init
    let game_id_regex = Regex::new(r#"Game (\d+)"#).unwrap();
    let color_regex = Regex::new(r#"(\d+) (red|green|blue)"#).unwrap();
    let mut all_games_summary = HashMap::<u32, Vec<(u32, u32, u32)>>::new();

    //Parse each game
    for game_results in all_results_iter {
        let mut game_rounds_results = Vec::<(u32, u32, u32)>::new();

        //Split between ID and game results
        let game_results_split = game_results.split(":").collect::<Vec<&str>>();

        //Get ID
        let id_match = game_id_regex.captures(game_results_split[0]).unwrap();
        let game_id = id_match
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        //Get game results
        for round_result in game_results_split[1].split(";") {
            let mut rgb_values = (0u32, 0u32, 0u32);

            for color_result in round_result.split(",") {
                let color_result_captures = color_regex.captures(color_result).unwrap();
                let color_count = color_result_captures
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();

                match color_result_captures.get(2).unwrap().as_str() {
                    "red" => rgb_values.0 = color_count,
                    "green" => rgb_values.1 = color_count,
                    "blue" => rgb_values.2 = color_count,
                    _ => panic!("Invalid color option")
                };

            }

            game_rounds_results.push(rgb_values);
        }

        all_games_summary.insert(game_id, game_rounds_results);
    }

    return all_games_summary;
}