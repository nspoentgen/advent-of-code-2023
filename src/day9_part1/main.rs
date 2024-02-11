use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};

fn main () {
    //Parse, calculate, and print answer
    let path = Path::new("src/day9_part1/input.txt");
    let next_line_value_sums = parse_data(&path)
        .iter()
        .map(|x| get_next_line_value(x))
        .sum::<i32>();

    println!("The sum is {}.", next_line_value_sums.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<Vec<i32>> {
    let file = File::open(&path).unwrap();
    let parse_line_lambda = |line: &String| -> Vec<i32> {
        return line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    };

    return BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_line_lambda(&l))
        .collect::<Vec<Vec<i32>>>();
}

fn get_next_line_value(line: &Vec<i32>) -> i32 {
    return generate_diff_tree(line)
        .iter()
        .map(|x| *x.last().unwrap())
        .sum();
}

fn generate_diff_tree(line: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut diff_tree = Vec::<Vec<i32>>::new();
    diff_tree.push(line.clone());

    while !diff_tree.last().unwrap().iter().all(|x| *x == 0) {
        let mut diff_layer = Vec::<i32>::new();
        let last_layer = diff_tree.last().unwrap();

        for index in 0..(last_layer.len() - 1) {
            diff_layer.push(last_layer[index + 1] - last_layer[index])
        }

        diff_tree.push(diff_layer);
    }

    return diff_tree;
}
