mod line_data;

use crate::line_data::*;
use itertools::{cloned, Itertools};
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::Path;
use lazy_static::lazy_static;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;

type CacheKey = (String, Vec<usize>, bool);

const NORMAL: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';
const LENGTHS_SEPARATOR: char = ',';

lazy_static!{
    static ref NORMAL_BASE_CASE_PATTERN: Vec<usize> = vec![0];
    static ref BROKEN_BASE_CASE_PATTERN: Vec<usize> = vec![1];
}

fn main() {
    //Parse data and calculated derived data
    let path = Path::new("src/day12_part1/input.txt");
    let all_data = parse_data(&path);

    let mut cache = HashMap::<CacheKey, usize>::new();
    let normal_match_count = calculate_continuous_broken_spring_lengths(all_data[0].get_status().to_owned(), all_data[0].get_continuous_broken_lengths().to_owned(), false, &mut cache);
    let broken_match_count = calculate_continuous_broken_spring_lengths(all_data[0].get_status().to_owned(), all_data[0].get_continuous_broken_lengths().to_owned(), true, &mut cache);
    let match_count = normal_match_count + broken_match_count;


    
    let mut foo = 1;
    foo += 1;
    foo += 1;
    foo += 1;
    foo += 1;
    foo += 1;

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

fn calculate_continuous_broken_spring_lengths(test_string: String, mut test_pattern: Vec<usize>, broken_start: bool, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let key = &(test_string.clone(), test_pattern.clone(), broken_start);
    if cache.contains_key(key) {
        return cache[key];
    }

    let match_count: usize;
    if test_string.len() > 1 {
        let local_spring = test_string.chars().nth(0).unwrap();
        let sub_test_string = test_string[1..test_string.len()].to_string();

        if local_spring == NORMAL {
            match_count = calculate_normal_case(sub_test_string, test_pattern, cache);
        } else if local_spring == BROKEN {
            match_count = calculate_broken_case(sub_test_string, test_pattern, cache);
        } else {
            match_count = calculate_normal_case(sub_test_string.clone(), test_pattern.clone(), cache) + calculate_broken_case(sub_test_string, test_pattern, cache);
        }
    } else {
        match_count = calculate_base_case(test_string, test_pattern, cache);
    }

    return match_count;
}

fn calculate_normal_case(test_string: String, test_pattern: Vec<usize>, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let normal_count = calculate_continuous_broken_spring_lengths(test_string.clone(), test_pattern.clone(), false, cache);
    let broken_count = calculate_continuous_broken_spring_lengths(test_string.clone(), test_pattern.clone(), true, cache);
    let match_count = normal_count + broken_count;

    cache.insert((test_string.clone(), test_pattern.clone(), false), normal_count);
    cache.insert((test_string, test_pattern, true), broken_count);

    return match_count;
}

fn calculate_broken_case(test_string: String, mut test_pattern: Vec<usize>, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let match_count: usize;

    if test_pattern[0] > 1 {
        test_pattern[0] -= 1;
        match_count = calculate_continuous_broken_spring_lengths(test_string.clone(), test_pattern.clone(), true, cache);

        cache.insert((test_string, test_pattern, true), match_count);
    } else if test_pattern.len() > 1 {
        let sub_test_pattern = test_pattern[1..test_pattern.len()].to_owned();

        let normal_count = calculate_continuous_broken_spring_lengths(test_string.clone(), sub_test_pattern.clone(), false, cache);
        let broken_count = calculate_continuous_broken_spring_lengths(test_string.clone(), sub_test_pattern.clone(), true, cache);
        match_count = normal_count + broken_count;

        cache.insert((test_string.clone(), sub_test_pattern.clone(), false), normal_count);
        cache.insert((test_string, sub_test_pattern, true), broken_count);
    } else {
        let sub_test_pattern = vec![0];
        match_count = calculate_continuous_broken_spring_lengths(test_string.clone(), sub_test_pattern.clone(), false, cache);

        cache.insert((test_string, sub_test_pattern, false), match_count);
    }

    return match_count;
}

fn calculate_base_case(test_string: String, test_pattern: Vec<usize>, cache: &mut HashMap<CacheKey, usize>) -> usize {
    if test_string.len() > 1 {
        panic!("Something is wrong. The test string length is > 1");
    }

    let match_count: usize;

    if test_pattern.len() > 1 {
        match_count = 0;
    } else if test_string.chars().nth(0).unwrap() == NORMAL {
        match_count = if test_pattern == *NORMAL_BASE_CASE_PATTERN { 1 } else { 0 };
    } else if test_string.chars().nth(0).unwrap() == BROKEN {
        match_count = if test_pattern == *BROKEN_BASE_CASE_PATTERN { 1 } else { 0 };
    } else {
        match_count = 1;
    }

    return match_count;
}
