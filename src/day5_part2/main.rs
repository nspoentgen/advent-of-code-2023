mod parse;
mod map;

use crate::parse::*;
use crate::map::*;

use std::path::Path;
use num_format::{Locale, ToFormattedString};


fn main() {
    //Program inputs
    let path = Path::new("src/day5_part1/input.txt");

    //Init data for parsing
    let mut seed_range_data = Vec::<(u64, u64)>::new();
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
    parse_data(&path, &mut seed_range_data, &mut maps_list);

    //Calculate min location
    let min_location = seed_range_data
        .iter()
        .flat_map(|x| get_output_ranges(x, &seed_soil_map))
        .flat_map(|x| get_output_ranges(&x, &soil_fertilizer_map))
        .flat_map(|x| get_output_ranges(&x, &fertilizer_water_map))
        .flat_map(|x| get_output_ranges(&x, &water_light_map))
        .flat_map(|x| get_output_ranges(&x, &light_temperature_map))
        .flat_map(|x| get_output_ranges(&x, &temperature_humidity_map))
        .flat_map(|x| get_output_ranges(&x, &humidity_location_map))
        .map(|x| x.0)
        .min()
        .unwrap();

    println!("The min location is {}", min_location.to_formatted_string(&Locale::en));
}
