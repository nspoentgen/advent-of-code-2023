use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use num_format::{Locale, ToFormattedString};

type Point = (usize, usize);
const GALAXY_SYMBOL: char = '#';
const EMPTY_SPACE_MULTIPLIER: usize = 1_000_000;

fn main() {
    //Read input
    let path = Path::new("src/day11_part1/input.txt");
    let file = File::open(&path).unwrap();
    let data =  BufReader::new(file).lines().flatten().collect::<Vec<String>>();

    //Find galaxies and empty regions. Hash empty regions for next step
    let galaxy_positions = find_galaxies(&data);
    let empty_rows = find_empty_rows(&data);
    let empty_cols = find_empty_cols(&data);
    let empty_rows_set = HashSet::<usize>::from_iter(empty_rows);
    let empty_cols_set = HashSet::<usize>::from_iter(empty_cols);

    //Generate expansion maps
    let max_row_index = data.len() - 1;
    let max_col_index = data[0].len() - 1;
    let expanded_row_map = generate_normal_to_expanded_map(&empty_rows_set, max_row_index);
    let expanded_col_map = generate_normal_to_expanded_map(&empty_cols_set, max_col_index);

    //Calculate distances
    let mut galaxy_pair_distances: Vec<u64> = vec![];

    for i in 0..galaxy_positions.len() - 1 {
        for j in i+1..galaxy_positions.len() {
            galaxy_pair_distances.push(calculate_distance(&galaxy_positions[i], &galaxy_positions[j],
                &expanded_row_map, &expanded_col_map));
        }
    }

    //Print final result
    println!("The sum of the galaxy pair distances is {}", galaxy_pair_distances.iter().sum::<u64>().to_formatted_string(&Locale::en));
}

fn find_galaxies(data: &Vec<String>) -> Vec<Point> {
    let mut galaxy_positions: Vec<Point> = vec![];

    for row_index in 0..data.len() {
        for col_index in 0..data[row_index].len() {
            if data[row_index].chars().nth(col_index).unwrap() == GALAXY_SYMBOL {
                galaxy_positions.push((row_index, col_index));
            }
        }
    }

    return galaxy_positions;
}

fn find_empty_rows(lines: &Vec<String>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = vec![];

    for row_index in 0..lines.len() {
        if !lines[row_index].contains(GALAXY_SYMBOL) {
            empty_rows.push(row_index);
        }
    }

    return empty_rows;
}

fn find_empty_cols(lines: &Vec<String>) -> Vec<usize> {
    let mut empty_cols: Vec<usize> = vec![];
    let num_cols = lines[0].len(); //all lines same length

    for col_index in 0..num_cols {
        let mut is_empty_col = true;

        for row_index in 0..lines.len() {
            if lines[row_index].chars().nth(col_index).unwrap() == GALAXY_SYMBOL {
                is_empty_col = false;
                break;
            }
        }

        if is_empty_col {
            empty_cols.push(col_index);
        }
    }

    return empty_cols;
}

fn generate_normal_to_expanded_map(empty_normal_indices: &HashSet<usize>, max_normal_index: usize) -> HashMap<usize, usize> {
    let mut expanded_dimension_map = HashMap::<usize, usize>::new();
    let mut cumulative_offset = 0usize;

    for normal_index in 0..=max_normal_index {
        if empty_normal_indices.contains(&normal_index) {
            cumulative_offset += EMPTY_SPACE_MULTIPLIER - 1;
        }

        expanded_dimension_map.insert(normal_index, normal_index + cumulative_offset);
    }

    return expanded_dimension_map;
}

fn calculate_distance(galaxy1_pos_normal: &Point, galaxy2_pos_normal: &Point, row_extension_map: &HashMap<usize, usize>,
                      col_extension_map: &HashMap<usize, usize>) -> u64 {

    let galaxy1_row_expanded = row_extension_map[&galaxy1_pos_normal.0];
    let galaxy1_col_expanded = col_extension_map[&galaxy1_pos_normal.1];
    let galaxy2_row_expanded = row_extension_map[&galaxy2_pos_normal.0];
    let galaxy2_col_expanded = col_extension_map[&galaxy2_pos_normal.1];

   return (galaxy2_row_expanded.abs_diff(galaxy1_row_expanded) + galaxy2_col_expanded.abs_diff(galaxy1_col_expanded)).try_into().unwrap();
}
