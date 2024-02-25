mod line_data;

use crate::line_data::*;
use itertools::{Itertools};
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::atomic::Ordering::Acquire;
use lazy_static::lazy_static;
use log::debug;

type TreePosition = (usize, bool);
type IntermediateResult = (Vec<usize>, bool);

const NORMAL: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';
const LENGTHS_SEPARATOR: char = ',';
static BROKEN_PATTERN_MATCH_COUNT: AtomicU32 = AtomicU32::new(0);

lazy_static! {
    pub static ref INITIAL_RESULT: (Vec<usize>, bool) = (Vec::<usize>::new(), true);
    pub static ref NORMAL_SPRING_OPTIONS: Vec<bool> = Vec::from([true, false]);
    pub static ref FINAL_SPRING_OPTIONS: Vec<bool> = Vec::from([true]);
}


fn main() {
    //Parse data and calculated derived data
    let path = Path::new("src/day12_part1/input.txt");
    let all_data = parse_data(&path);
    let mut result_cache = HashMap::<TreePosition, IntermediateResult>::new();
    calculate_continuous_broken_spring_lengths(&all_data[1].get_status(), 0, true, &mut result_cache, &all_data[1].get_continuous_broken_lengths());
    println!("The count is {}", BROKEN_PATTERN_MATCH_COUNT.load(Acquire))

    /*
        //Calculate result and print answer
    let match_sums = all_data
        .iter()
        .map(|x| calc_num_matches(x))
        .sum::<usize>();
    println!("The match sum total is {}", match_sums.to_formatted_string(&Locale::en));
     */

}

fn parse_data(path: &Path) -> Vec<LineData> {
    let mut all_data = Vec::<LineData>::new();

    let file = File::open(&path).unwrap();
    for line in BufReader::new(file).lines().flatten() {
        let line_parts = line.split(" ").collect_vec();


        let status = line_parts[0].to_string();
        let expanded_status = vec![status; 5].join("?");
        let expanded_continuous_broken_lengths = vec![line_parts[1]; 5]
            .join(",")
            .split(&LENGTHS_SEPARATOR.to_string())
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();

        all_data.push(LineData::new(expanded_status, expanded_continuous_broken_lengths));
    }

    return all_data;
}

fn calculate_continuous_broken_spring_lengths(status: &String, offset: usize, parent_path: bool,
    result_cache: &mut HashMap<TreePosition, IntermediateResult>, match_lengths: &Vec<usize>)
{
    //Init
    let substring = &status[offset..];
    let match_result = substring.find(&UNKNOWN.to_string());
    let test_options: &Vec<bool> = if match_result.is_some() { &NORMAL_SPRING_OPTIONS } else { &FINAL_SPRING_OPTIONS };

    //Loop through valid path options. True = normal spring, false = broken
    for spring_option in test_options {
        // Calculate the cumulative result for this iteration
        let (mut local_broken_spring_lengths, end_index, normal_end) = calculate_local_result(&status, offset, &match_result, *spring_option);
        let parent_result = if offset == 0 { &INITIAL_RESULT } else { &result_cache[&(offset - 1, parent_path)] };
        let cumulative_broken_lengths = calculate_cumulative_result(&parent_result, &mut local_broken_spring_lengths, substring);

        //Cache the result
        result_cache.insert((end_index, *spring_option), (cumulative_broken_lengths, normal_end));
        let iteration_result = &result_cache[&(end_index, *spring_option)];

        //Recurse or terminate
        if end_index + 1 <= status.len() - 1 {
           if is_possible_path(&iteration_result.0, match_lengths) {
                calculate_continuous_broken_spring_lengths(&status, end_index + 1, *spring_option, result_cache, match_lengths);
           }
        } else {
            if *iteration_result.0 == *match_lengths {
                BROKEN_PATTERN_MATCH_COUNT.fetch_add(1, Acquire);
            }
        }
    }
}

fn calculate_local_result(status: &String, offset:usize, unknown_match_result: &Option<usize>, spring_option: bool) -> (Vec<usize>, usize, bool) {
    let test_string: String;
    let end_index: usize;

    match *unknown_match_result {
        //Unknown here means spring unknown character
        Some(unknown_substring_index) => {
            let unknown_status_index = offset + unknown_substring_index;
            test_string = (&status[offset..=unknown_status_index - 1]).to_owned() + &if spring_option { NORMAL.to_string() } else { BROKEN.to_string() };
            end_index = unknown_status_index;
        },
        None => {
            test_string = status[offset..].to_owned();
            end_index = status.len() - 1;
        }
    };

    let local_lengths = test_string
        .split(&NORMAL.to_string())
        .filter(|x| !x.is_empty())
        .map(|x| x.len())
        .collect_vec();
    let normal_end = test_string.chars().last().unwrap() == NORMAL;

    return (local_lengths, end_index, normal_end);
}

fn calculate_cumulative_result(parent_result: &(Vec<usize>, bool), local_broken_spring_lengths: &mut Vec<usize>,
    substring: &str) -> Vec<usize>
{
    return match parent_result.1 {
        true => {
            let mut temp = parent_result.0.clone();
            temp.append(local_broken_spring_lengths);

            temp
        },
        false => {
            match local_broken_spring_lengths.len() {
                0 => parent_result.0.to_owned(),
                _ => {
                    let mut temp = parent_result.0[0..parent_result.0.len() - 1].to_owned();

                    if substring.chars().nth(0).unwrap() == NORMAL {
                        temp.push(parent_result.0[parent_result.0.len() - 1]);
                        temp.push(local_broken_spring_lengths[0]);
                    } else {
                        temp.push(parent_result.0[parent_result.0.len() - 1] + local_broken_spring_lengths[0]);
                    }

                    if local_broken_spring_lengths.len() >= 2 {
                        temp.extend_from_slice(&local_broken_spring_lengths[1..]);
                    }

                    temp
                }
            }
        }
    };
}

fn is_possible_path(current_lengths: &Vec<usize>, all_lengths: &Vec<usize>) -> bool {
    let broken_vec_length = current_lengths.len();

    if broken_vec_length == 0 {
        return true;
    }

    if broken_vec_length > all_lengths.len() {
        return false;
    }

    let mut possible = true;
    if broken_vec_length > 1 {
        for index in 0..broken_vec_length - 1 {
            possible &= current_lengths[index] == all_lengths[index];
        }
    }
    possible &= current_lengths[broken_vec_length - 1] <= all_lengths[broken_vec_length - 1];

    return possible;
}
