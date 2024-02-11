use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use num_format::{Locale, ToFormattedString};

type Point = (usize, usize);
const EMPTY_SYMBOL: char = '.';
const GALAXY_SYMBOL: char = '#';

fn main() {
    //Read input
    let path = Path::new("src/day11_part1/input.txt");
    let file = File::open(&path).unwrap();
    let mut data =  BufReader::new(file).lines().flatten().collect::<Vec<String>>();

    //Find empty regions and insert extra space
    let empty_rows = find_empty_rows(&data);
    let empty_cols = find_empty_cols(&data);
    add_extra_rows(&mut data, &empty_rows);
    add_extra_cols(&mut data, &empty_cols);

    //Find galaxy positions and calculate distances
    let galaxy_positions = find_galaxies(&data);
    let mut galaxy_pair_distances: Vec<u32> = vec![];

    for i in 0..galaxy_positions.len() - 1 {
        for j in i+1..galaxy_positions.len() {
            galaxy_pair_distances.push(calculate_distance(&galaxy_positions[i], &galaxy_positions[j]));
        }
    }

    //Print final result
    println!("The sum of the galaxy pair distances is {}", galaxy_pair_distances.iter().sum::<u32>().to_formatted_string(&Locale::en));
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

//Assumes empty row indices sorted
fn add_extra_rows(lines: &mut Vec<String>, empty_row_indices: &Vec<usize>) {
    let blank_line: String = vec![EMPTY_SYMBOL; lines[0].len()].iter().collect();
    let mut row_offset = 0usize;

    for empty_row_index in empty_row_indices {
        lines.insert(empty_row_index + row_offset, blank_line.clone());
        row_offset += 1;
    }
}

//Assumes empty col indices sorted
fn add_extra_cols(lines: &mut Vec<String>, empty_col_indices: &Vec<usize>) {
    let mut col_offset = 0usize;

    for empty_col_index in empty_col_indices {
        for row_index in 0..lines.len() {
            lines[row_index].insert(empty_col_index + col_offset, EMPTY_SYMBOL);
        }

        col_offset += 1;
    }
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

fn calculate_distance(galaxy1_pos: &Point, galaxy2_pos: &Point) -> u32 {

    return (galaxy2_pos.0.abs_diff(galaxy1_pos.0) + galaxy2_pos.1.abs_diff(galaxy1_pos.1)).try_into().unwrap();
}
