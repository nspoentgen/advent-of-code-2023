use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use itertools::Itertools;

pub fn parse_data(path: &Path, seed_ranges: &mut Vec<(u64, u64)>, maps_list: &mut [&mut Vec<(u64, u64, u64)>; 7]) {
    //Get line iterator into file
    let file = File::open(&path).unwrap();
    let mut line_iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .into_iter();

    //Extract seed list
    *seed_ranges = parse_seed_ranges(&mut line_iter);

    //Extract maps
    parse_maps(&mut line_iter, maps_list);
}

pub fn parse_seed_ranges(line_iter: &mut impl Iterator<Item=String>) -> Vec<(u64, u64)> {
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
        .chunks(2)
        .into_iter()
        .map(|x| {
            let seed_range_data = x.collect::<Vec<u64>>();
            return (seed_range_data[0], seed_range_data[1]);
        })
        .collect::<Vec<(u64, u64)>>();

    //Consume blank line before returning
    line_iter.next();
    return seed_list;
}

pub fn parse_maps(
    line_iter: &mut impl Iterator<Item=String>,
    maps_list: &mut [&mut Vec<(u64, u64, u64)>; 7]
) {
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
