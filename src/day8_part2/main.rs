mod step_options;
mod scalar_solver_iterator;

use crate::step_options::StepOptions;
use crate::scalar_solver_iterator::*;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use itertools::Itertools;
use num::integer::lcm;
use num_format::{Locale, ToFormattedString};

fn main () {
    //Parse data
    let path = Path::new("src/day8_part1/input.txt");
    let (instructions, desert_map) = parse_data(&path);

    //Print final answer
    let num_map_steps = calculate_num_map_steps(&instructions, &desert_map);
    println!("The number of map steps = {}", num_map_steps.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> (String, HashMap<String, StepOptions>) {
    let file = File::open(&path).unwrap();
    let mut lines_iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    let instructions = lines_iter.next().unwrap();
    lines_iter.next(); //consume blank line
    let desert_map = parse_map_data(&mut lines_iter);

    return (instructions, desert_map);
}

fn parse_map_data(lines_iter: &mut impl Iterator<Item=String>) -> HashMap<String, StepOptions> {
    let mut desert_map = HashMap::<String, StepOptions>::new();

    while let Some(line) = lines_iter.next() {
        let node_raw_parts = line
            .split("=")
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();
        let node_code = node_raw_parts[0].clone();
        let left_step = node_raw_parts[1][1..4].to_string();
        let right_stp = node_raw_parts[1][6..9].to_string();
        let step_option = StepOptions {
            left: left_step,
            right: right_stp
        };
        desert_map.insert(node_code, step_option);
    }

    return desert_map;
}

fn calculate_num_map_steps(instructions: &String, desert_map: &HashMap<String, StepOptions>) -> u64 {
    let starting_vector = desert_map
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&String>>();

    let mut solution_iterators = starting_vector
        .iter()
        .map(|x| ScalarSolverIterator::new(x, instructions, desert_map))
        .collect::<Vec<ScalarSolverIterator>>();

    //Investigation found that each solution is stuck on a loop and each solution is on the same index but different
    //iteration count. Multiply the LCM of all solution iterations * instruction length + 1 copy of the index
    let iterations = solution_iterators
        .iter_mut()
        .map(|mut x| x.next().unwrap().iteration)
        .collect::<Vec<u64>>();

    let mut lcm_value = iterations[0];
    for index in 1..iterations.len() {
        lcm_value = lcm(lcm_value, iterations[index]);
    }

    return lcm_value * instructions.len() as u64 + solution_iterators[0].next().unwrap().index as u64;
}


