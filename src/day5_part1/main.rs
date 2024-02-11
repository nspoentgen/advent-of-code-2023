use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use num_format::{Locale, ToFormattedString};


fn main() {
    //Program inputs
    let path = Path::new("src/day5_part1/input.txt");

    //Init data for parsing
    let mut seeds = Vec::<u64>::new();
    let mut seed_soil_map = Vec::<(u64, u64, u64)>::new();
    let mut soil_fertilizer_map =  Vec::<(u64, u64, u64)>::new();
    let mut fertilizer_water_map = Vec::<(u64, u64, u64)>::new();
    let mut water_light_map= Vec::<(u64, u64, u64)>::new();
    let mut light_temperature_map = Vec::<(u64, u64, u64)>::new();
    let mut temperature_humidity_map = Vec::<(u64, u64, u64)>::new();
    let mut humidity_location_map = Vec::<(u64, u64, u64)>::new();
    let mut maps_list = [
        &mut seed_soil_map,
        &mut soil_fertilizer_map,
        &mut fertilizer_water_map,
        &mut water_light_map,
        &mut light_temperature_map,
        &mut temperature_humidity_map,
        &mut humidity_location_map
        ];

    //Parse data
    parse_data(&path, &mut seeds, &mut maps_list);

    //Map over data to get to get min location
    let min_location = seeds
        .iter()
        .map(|x| get_mapped_output(x.clone(), &seed_soil_map))
        .map(|x| get_mapped_output(x, &soil_fertilizer_map))
        .map(|x| get_mapped_output(x, &fertilizer_water_map))
        .map(|x| get_mapped_output(x, &water_light_map))
        .map(|x| get_mapped_output(x, &light_temperature_map))
        .map(|x| get_mapped_output(x, &temperature_humidity_map))
        .map(|x| get_mapped_output(x, &humidity_location_map))
        .min()
        .unwrap();

    println!("The min location is {}", min_location.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path, seeds: &mut Vec<u64>, maps_list: &mut [&mut Vec<(u64, u64, u64)>; 7]) {
    //Get line iterator into file
    let file = File::open(&path).unwrap();
    let mut line_iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    //Extract seed list
    *seeds = parse_seeds(&mut line_iter);

    //Extract maps
    parse_maps(&mut line_iter, maps_list);
}

fn parse_seeds(line_iter: &mut impl Iterator<Item=String>) -> Vec<u64> {
    let unwrapped_line = line_iter
        .next()
        .unwrap();

    let seed_list_raw = unwrapped_line
        .split(":")
        .collect::<Vec<&str>>()[1];

    let seed_list = seed_list_raw
        .trim()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    //Consume blank line before returning
    line_iter.next();
    return seed_list;
}

fn parse_maps(line_iter: &mut impl Iterator<Item=String>, maps_list: &mut [&mut Vec<(u64, u64, u64)>; 7]) {
    //For each map block, discard the header then parse the map data. Consume final blank line before
    //moving on to next map parse.
    for map in maps_list {
        line_iter.next();

        loop {
            let map_line = line_iter.next().unwrap_or_else(|| "".to_string());

            if map_line.is_empty() {
                break;
            } else {
                let map_data = map_line
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let value_start = map_data[0];
                let key_start = map_data[1];
                let key_length = map_data[2];
                map.push((key_start, value_start, key_length));
            }
        }
    }
}

fn get_mapped_output(input: u64, map_list: &Vec<(u64, u64, u64)>) -> u64 {
    match find_map_index(input, &map_list) {
        Some(map_index) => {
            let map_data = map_list[map_index];
            apply_map(input, map_data.0, map_data.1, map_data.2)
        },
        None => input
    }
}

fn find_map_index(input: u64, map_list: &Vec<(u64, u64, u64)>) -> Option<usize> {
    for (index, map) in map_list.iter().enumerate() {
        if input >= map.0 && input < map.0 + map.2 {
            return Some(index);
        }
    }

    return None;
}

fn apply_map(x: u64, x0: u64, y0: u64, length: u64) -> u64 {
    if x >= x0 + length {
        panic!("Can't map");
    }

    return y0 + (x - x0);
}
