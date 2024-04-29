use std::path::Path;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::modules::*;

mod modules;

fn main() {

}

fn parse_data(path: &Path) -> Vec<Box<dyn Module>> {
    let mut modules = Vec::<Box<dyn Module>>::new();
    let file = File::open(&path).unwrap();

    for line in BufReader::new(file).lines().flatten() {
        let line_split: Vec<&str> = line.split("->").collect();
        let mut module_type_definition = line_split[0];
        let destinations: Vec<String> = line_split[1].split(",")
            .into_iter()
            .map(|x| x.trim().to_string())
            .collect();

        module_type_definition = module_type_definition.trim();
        if module_type_definition == Broadcaster::NAME {
            modules.push(Box::new(Broadcaster::new(&destinations)));
        } else if module_type_definition.chars().nth(0).unwrap() == '%' {
            modules.push(Box::new(FlipFlop::new(&destinations)));
        } else if module_type_definition.chars().nth(0).unwrap() == '&' {
            modules.push(Box::new(Conjunction::new(&destinations)))
        }
    }

    return modules;
}

