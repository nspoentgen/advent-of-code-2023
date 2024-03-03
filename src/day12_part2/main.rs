mod line_data;

#[macro_use]
extern crate timeit;

use crate::line_data::*;
use itertools::{Itertools};
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use lazy_static::lazy_static;
use num_format::{Locale, ToFormattedString};

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
    timeit!({
        //Parse data and calculated derived data
        let path = Path::new("src/day12_part1/input.txt");
        let all_data = parse_data(&path);

        //Calculate result and print answer
        let mut cache = HashMap::<CacheKey, usize>::new();
        let match_sums = all_data
            .iter()
            .map(|x| calculate_continuous_broken_spring_lengths(x.get_status().clone(), x.get_continuous_broken_lengths().clone(), false, &mut cache) +
                calculate_continuous_broken_spring_lengths(x.get_status().clone(), x.get_continuous_broken_lengths().clone(), true, &mut cache))
            .sum::<usize>();

        println!("The match sum total is {}", match_sums.to_formatted_string(&Locale::en));
    });
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

fn calculate_continuous_broken_spring_lengths(test_string: String, test_pattern: Vec<usize>, broken_start: bool, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let key = &(test_string.clone(), test_pattern.clone(), broken_start);
    if cache.contains_key(key) {
        return cache[key];
    }

    let match_count: usize;

    if broken_start && test_string.chars().nth(0).unwrap() == NORMAL {
        match_count = 0;
    } else if !broken_start && test_string.chars().nth(0).unwrap() == BROKEN {
        match_count = 0;
    }
    else {
        let local_spring = test_string.chars().nth(0).unwrap();

        if local_spring == NORMAL {
            match_count = calculate_normal_case(test_string.clone(), test_pattern.clone(), cache);
        } else if local_spring == BROKEN {
            match_count = calculate_broken_case(test_string.clone(), test_pattern.clone(), cache);
        } else {
            let mut normal_test_string = test_string.clone();
            let mut broken_test_string = test_string.clone();
            normal_test_string.replace_range(0..1, &NORMAL.to_string());
            broken_test_string.replace_range(0..1, &BROKEN.to_string());

            match_count = calculate_continuous_broken_spring_lengths(normal_test_string, test_pattern.clone(), broken_start, cache) +
                calculate_continuous_broken_spring_lengths(broken_test_string, test_pattern.clone(), broken_start, cache)
        }
    }

    //println!("Test string = {}, test pattern {:?}, broken start = {}, count = {}", &test_string, &test_pattern, broken_start, &match_count);
    return match_count;
}

fn calculate_normal_case(test_string: String, test_pattern: Vec<usize>, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let match_count: usize;

    if test_string.len() > 1 {
        let sub_test_string = test_string[1..test_string.len()].to_string();
        let normal_count = calculate_continuous_broken_spring_lengths(sub_test_string.clone(), test_pattern.clone(), false, cache);
        let broken_count = calculate_continuous_broken_spring_lengths(sub_test_string.clone(), test_pattern.clone(), true, cache);

        match_count = normal_count + broken_count;
        conditionally_update_cache(cache, (sub_test_string.clone(), test_pattern.clone(), false), normal_count);
        conditionally_update_cache(cache, (sub_test_string, test_pattern, true), broken_count);
    } else {
        match_count = if test_pattern == *NORMAL_BASE_CASE_PATTERN { 1 } else { 0 };
        conditionally_update_cache(cache, (test_string, test_pattern, false), match_count);
    }

    return match_count;
}

fn calculate_broken_case(test_string: String, mut test_pattern: Vec<usize>, cache: &mut HashMap<CacheKey, usize>) -> usize {
    let match_count: usize;

    if test_pattern == *NORMAL_BASE_CASE_PATTERN {
        match_count = 0;
    }
    else if test_string.len() > 1 {
        let sub_test_string = test_string[1..test_string.len()].to_string();

        if test_pattern[0] > 1 {
            test_pattern[0] -= 1;
            match_count = calculate_continuous_broken_spring_lengths(sub_test_string.clone(), test_pattern.clone(), true, cache);
            conditionally_update_cache(cache, (sub_test_string, test_pattern, true), match_count);
        } else if test_pattern.len() > 1 {
            let sub_test_pattern = test_pattern[1..test_pattern.len()].to_owned();
            match_count = calculate_continuous_broken_spring_lengths(sub_test_string.clone(), sub_test_pattern.clone(), false, cache);
            conditionally_update_cache(cache, (sub_test_string.clone(), sub_test_pattern.clone(), false), match_count);
        } else {
            let sub_test_pattern = NORMAL_BASE_CASE_PATTERN.clone();
            match_count = calculate_continuous_broken_spring_lengths(sub_test_string.clone(), sub_test_pattern.clone(), false, cache);
            conditionally_update_cache(cache, (sub_test_string, sub_test_pattern, false), match_count);
        }
    } else {
        match_count = if test_pattern == *BROKEN_BASE_CASE_PATTERN { 1 } else { 0 };
        conditionally_update_cache(cache, (test_string, test_pattern, true), match_count);
    }

    return match_count;
}

fn conditionally_update_cache(cache: &mut HashMap<CacheKey, usize>, key: CacheKey, value: usize) {
    if !cache.contains_key(&key) {
        cache.insert(key, value);
    }
}
