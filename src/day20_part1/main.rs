use std::path::Path;
use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::fs::File;
use std::io::{BufRead, BufReader};
use num_format::{Locale, ToFormattedString};

use crate::modules::*;

mod modules;

fn main() {
    let path = Path::new("src/day20_part1/input.txt");
    let mut downstream_modules = parse_data(&path);

    let mut low_pulse_count = 0u64;
    let mut high_pulse_count = 0u64;

    for _ in 0..1000 {
        (low_pulse_count, high_pulse_count, downstream_modules) = process_one_cycle(low_pulse_count, high_pulse_count, downstream_modules);
    }

    println!("Low * high pulse count = {}", (low_pulse_count * high_pulse_count).to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> HashMap<String, Box<dyn PulseReceiver>> {
    //Init
    let mut modules = HashMap::<String, Box<dyn PulseReceiver>>::new();
    let file = File::open(&path).unwrap();
    let mut io_map = HashMap::<String, Vec<String>>::new();
    let mut conjunction_destinations = HashMap::<String, Vec<String>>::new();

    //Closure for storing relationships. Needed later for updating inputs
    //for conjunctions
    let mut update_io_map = |source: &str, destinations: &Vec<String>| {
        for destination in destinations {
            if !io_map.contains_key(destination) {
                io_map.insert(destination.clone(), vec![source.to_string()]);
            } else {
                io_map.get_mut(destination).unwrap().push(source.to_string());
            }
        }
    };

    //Define each module. Conjunctions are special and are only partially defined.
    //They will be updated in a following step.
    for line in BufReader::new(file).lines().flatten() {
        let line_split: Vec<&str> = line.split("->").collect();
        let mut module_type_definition = line_split[0];
        let destinations: Vec<String> = line_split[1].split(",")
            .into_iter()
            .map(|x| x.trim().to_string())
            .collect();

        module_type_definition = module_type_definition.trim();
        if module_type_definition == Broadcaster::NAME {
            modules.insert(Broadcaster::NAME.to_string(), Box::new(Broadcaster::new(&destinations)));
            update_io_map(Broadcaster::NAME, &destinations);

        } else if module_type_definition.chars().nth(0).unwrap() == '%' {
            let name = &module_type_definition[1..];
            modules.insert(name.to_string(), Box::new(FlipFlop::new(name, &destinations)));
            update_io_map(name, &destinations);

        } else if module_type_definition.chars().nth(0).unwrap() == '&' {
            let name = &module_type_definition[1..];
            update_io_map(name, &destinations);
            conjunction_destinations.insert(name.to_string(), destinations);
        }
    }

    //Create conjunction modules
    for (name, destinations) in &conjunction_destinations {
        modules.insert(name.clone(), Box::new(Conjunction::new(name, &io_map[name], destinations)));
    }

    return modules;
}

fn process_one_cycle(initial_low_pulse_count: u64, initial_high_pulse_count: u64, mut downstream_modules: HashMap<String, Box<dyn PulseReceiver>>) -> (u64, u64, HashMap<String, Box<dyn PulseReceiver>>) {
    let mut low_pulse_count = initial_low_pulse_count;
    let mut high_pulse_count = initial_high_pulse_count;
    let mut pulse_queue = VecDeque::<PulseOutput>::new();

    //We always start with a single assumed button module and then go from there
    let button_module = Button {};
    let (low_pulses_generated, high_pulses_generated, output_pulses) = button_module.push();
    low_pulse_count += low_pulses_generated;
    high_pulse_count += high_pulses_generated;
    pulse_queue.extend(output_pulses);

    //Keep processing pulses as long as they are being generated. Process in FIFO order per
    //problem statement. There are module sinks that can exist. We can detect these by noticing
    //that there is no downstream module registered with the given name.
    while pulse_queue.len() > 0 {
        let pulse = pulse_queue.pop_front().unwrap();
        let (low_pulses_generated, high_pulses_generated, output_pulses) = match downstream_modules.entry(pulse.1) {
            Entry::Occupied(o) => {
                let destination = o.into_mut();
                destination.process_input_pulse(&pulse.0, pulse.2)
            },
            Entry::Vacant(_) => (0u64, 0u64, vec![])
        };



        low_pulse_count += low_pulses_generated;
        high_pulse_count += high_pulses_generated;
        pulse_queue.extend(output_pulses);
    }

    return (low_pulse_count, high_pulse_count, downstream_modules);
}
