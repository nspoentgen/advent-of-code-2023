use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_format::{Locale, ToFormattedString};

mod utility;
use crate::utility::*;

fn main() {
    //Constants
    const NUM_BOXES: usize = 256;

    //Parse data
    let path = Path::new("src/day15_part1/input.txt");
    let steps = parse_data(&path);

    //Execute steps
    let mut lens_boxes = vec![LensBox::new(); NUM_BOXES];
    execute_steps(&mut lens_boxes, &steps);

    //Calculate combined power
    let mut combined_power = 0usize;
    for (box_index, lens_box) in lens_boxes.iter().enumerate()  {
        for (lens_index, lens) in lens_box.lenses.iter().enumerate() {
            combined_power += calculate_lens_power(box_index, lens_index, lens.focal_length);
        }
    }

    //Print the result
    println!("The combined power is {}", combined_power.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<Step> {
    //Constants
    const SPLIT_CHAR: char = ',';
    const INSERTION_SYMBOL: char = '=';
    const DELETION_SYMBOL: char = '-';

    //Single line file so casting vector to scalar string
    let file = File::open(&path).unwrap();
    let raw_data = &BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()[0];

    //Convert encoded step data to step objects
    let mut parsed_data = Vec::<Step>::new();
    for encoded_step in raw_data.split(SPLIT_CHAR) {
        let (operation_index, is_insertion) = if let Some(index) = encoded_step.find(INSERTION_SYMBOL) {
            (index, true)
        } else {
            (encoded_step.find(DELETION_SYMBOL).unwrap(), false)
        };

        let lens_label = encoded_step[0..operation_index].to_string();
        let step = if is_insertion {
            Step {
                lens_label: lens_label.clone(),
                box_index: calculate_label_hash(&lens_label),
                operation: Operation::Insert,
                lens_focal_length: Some(encoded_step[operation_index+1..].parse::<u32>().unwrap()),
            }
        } else {
            Step {
                lens_label: lens_label.clone(),
                box_index: calculate_label_hash(&lens_label),
                operation: Operation::Remove,
                lens_focal_length: None
            }
        };

        parsed_data.push(step);
    }

    return parsed_data;
}

fn calculate_label_hash(step: &str) -> usize {
    const INITIAL_VALUE: usize = 0;
    const MULTIPLIER: usize = 17;
    const MODULUS: usize = 256;

    return step
        .chars()
        .fold(INITIAL_VALUE, |acc, x| {
            let mut temp = acc + x as usize;
            temp *= MULTIPLIER;
            return temp % MODULUS;
        });
}

fn execute_steps(boxes: &mut Vec<LensBox>, steps: &Vec<Step>) {
    for step in steps {
        match step.operation {
            Operation::Insert => {
                let insertion_lens = Lens { label: step.lens_label.clone(), focal_length: step.lens_focal_length.unwrap() };

                let result = boxes[step.box_index].lenses.replace_first(
                    |x: &Lens| x.label == step.lens_label,
                    insertion_lens.clone());

                if result.is_none() {
                    boxes[step.box_index].lenses.push(insertion_lens);
                }
            },
            Operation::Remove => {
                boxes[step.box_index].lenses.remove_first(|x: &Lens| x.label == step.lens_label);
            }
        };
    }
}

fn calculate_lens_power(box_index: usize, lens_index: usize, lens_focal_length: u32) -> usize {
    return (box_index + 1) * (lens_index + 1) * (lens_focal_length as usize);
}