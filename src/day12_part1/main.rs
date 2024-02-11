use itertools::{Itertools};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

const NORMAL: char = '.';
const BROKEN: char = '#';
const UNKNOWN: char = '?';
static NUM_LINES_PROCESSED: AtomicU32 = AtomicU32::new(0);

struct LineData {
    status: String,
    continuous_broken_lengths: Vec<usize>,
    unknown_indices: Vec<usize>
}

impl LineData {
    fn get_status<'a>(&'a self) -> &'a String {
        return &self.status;
    }

    fn get_continuous_broken_lengths<'a>(&'a self) -> &'a Vec<usize> {
        return &self.continuous_broken_lengths;
    }

    fn get_unknown_indices<'a>(&'a self) -> &'a Vec<usize> {
        return &self.unknown_indices;
    }

    fn new(status: String, continuous_broken_lengths: Vec<usize>) -> Self {
        let unknown_indices = find_unknown_indices(&status);
        return Self {status, continuous_broken_lengths, unknown_indices};
    }
}

fn main() {
    //Parse data and calculated derived data
    let path = Path::new("src/day12_part1/input.txt");
    let all_data = parse_data(&path);

    unsafe {
        //Calculate result and print answer
        let match_sums = all_data
            .par_iter()
            .map(|x| calc_num_matches(x))
            .sum::<usize>();
        println!("The match sum total is {}", match_sums.to_formatted_string(&Locale::en));
    }

}

fn parse_data(path: &Path) -> Vec<LineData> {
    let mut all_data = Vec::<LineData>::new();

    let file = File::open(&path).unwrap();
    for line in BufReader::new(file).lines().flatten() {
        let line_parts = line.split(" ").collect_vec();
        let status = line_parts[0].to_string();
        let continuous_broken_lengths = line_parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();

        all_data.push(LineData::new(status, continuous_broken_lengths));
    }

    return all_data;
}

fn find_unknown_indices(status_string: &String) -> Vec<usize> {
    return status_string
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == UNKNOWN)
        .map(|(i, _)| i)
        .collect_vec();
}

fn generate_boolean_vector_permutations(size: usize) -> Vec<Vec<bool>> {
    let base_case = vec![true, false];
    return (0..size)
        .map(|_| base_case.clone())
        .multi_cartesian_product()
        .collect_vec();
}

fn generate_test_string(test_case: &Vec<bool>, unknown_indices: &Vec<usize>, line: &String) -> String {
    let mut test_string_builder = Vec::<char>::new();
    let unknown_indices_set = HashSet::<usize>::from_iter(unknown_indices.clone().into_iter());
    let mut test_case_iter = test_case.iter();

    for i in 0..line.len() {
        let value = if unknown_indices_set.contains(&i) {
            if *test_case_iter.next().unwrap() {NORMAL} else {BROKEN}
        } else {
            line.chars().nth(i).unwrap()
        };

        test_string_builder.push(value)
    }

    return String::from_iter(test_string_builder);
}

unsafe fn calc_num_matches(line_data: &LineData) -> usize {
    let test_case_is_match = |test_case| -> bool {
        let test_string = generate_test_string(&test_case, line_data.get_unknown_indices(), line_data.get_status());
        let test_continuous_broken_lengths = test_string
            .split(&NORMAL.to_string())
            .filter(|x| !x.is_empty())
            .map(|x| x.len())
            .collect_vec();

        return test_continuous_broken_lengths == line_data.continuous_broken_lengths;
    };

    let previous_value = NUM_LINES_PROCESSED.fetch_add(1, Ordering::Acquire);
    println!("Num lines processed = {}", previous_value + 1);

    let matches = generate_boolean_vector_permutations(line_data.get_unknown_indices().len())
        .into_iter()
        .map(|test_case| test_case_is_match(test_case))
        .filter(|x| *x)
        .collect_vec();

    return matches.len();
}
