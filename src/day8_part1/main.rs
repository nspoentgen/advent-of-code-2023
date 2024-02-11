mod step_options;

use crate::step_options::StepOptions;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::panic::panic_any;
use std::path::Path;
use num_format::Locale::lag;
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

fn calculate_num_map_steps(instructions: &String, desert_map: &HashMap<String, StepOptions>) -> u32 {
    let final_node: String = String::from("ZZZ");
    let mut num_steps = 0u32;
    let mut instruction_index = 0usize;
    let mut current_node = &String::from("AAA");

    while *current_node != final_node {
        let next_step = instructions
            .chars()
            .nth(instruction_index % instructions.len())
            .unwrap();

        current_node = match next_step {
            'L' => &desert_map[current_node].left,
            'R' => &desert_map[current_node].right,
            _ => panic!("Cannot map instruction")
        };

        instruction_index += 1;
        num_steps += 1;
    }

    return num_steps;
}
