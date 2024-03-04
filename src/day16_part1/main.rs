use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

use BeamDirection::*;

const EMPTY: char = '.';
const REFLECTOR_45_DEG: char = '/';
const REFLECTOR_135_DEG: char = '\\';
const VERTICAL_SPLITTER: char = '|';
const HORIZONTAL_SPLITTER: char = '-';

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BeamDirection { North, East, South, West }

fn main() {
    //Constants
    const STARTING_POS: (usize, usize) = (0, 0);
    const STARTING_BEAM_DIRECTION: BeamDirection = East;

    //Parse data
    let path = Path::new("src/day16_part1/input.txt");
    let feature_map = parse_data(&path);

    //Trace beam
    let mut energized_tiles = HashSet::<(usize, usize, BeamDirection)>::new();
    trace_beam(&STARTING_POS, STARTING_BEAM_DIRECTION, &feature_map, &mut energized_tiles);

    //Print result
    let num_energized_tiles = energized_tiles
        .iter()
        .map(|x| (x.0, x.1))
        .unique()
        .count();
    println!("Number of energized tiles = {}", num_energized_tiles.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<Vec<char>> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
}

fn trace_beam(starting_pos: &(usize, usize), mut beam_direction: BeamDirection, feature_map: &Vec<Vec<char>>, energized_tiles: &mut HashSet<(usize, usize, BeamDirection)>) {
    let row_max = feature_map.len() - 1;
    let col_max = feature_map[0].len() - 1;
    let mut complete = false;
    let mut current_pos = starting_pos.clone();


    while !complete {
        energized_tiles.insert((current_pos.0, current_pos.1, beam_direction));

        //debug_print(energized_tiles, row_max, col_max);
        //print!("\n");

        let feature = feature_map[current_pos.0][current_pos.1];

        if feature == EMPTY {
            (current_pos, complete) = handle_empty_case(&current_pos, beam_direction, row_max, col_max, energized_tiles);
        } else if feature == REFLECTOR_45_DEG {
            (current_pos, beam_direction, complete) = handle_reflector_45_deg_case(&current_pos, beam_direction, row_max, col_max, energized_tiles);
        } else if feature == REFLECTOR_135_DEG {
            (current_pos, beam_direction, complete) = handle_reflector_135_deg_case(&current_pos, beam_direction, row_max, col_max, energized_tiles);
        } else if feature == VERTICAL_SPLITTER {
            (current_pos, complete) = handle_vertical_splitter_case(&current_pos, beam_direction, row_max, col_max, feature_map, energized_tiles);
        } else {
            (current_pos, complete) = handle_horizontal_splitter_case(&current_pos, beam_direction, row_max, col_max, feature_map, energized_tiles);
        }
    }
}

fn handle_empty_case(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
    energized_tiles: &HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), bool)
{
    return if let Some(next_pos) = checked_advance(pos, beam_direction, row_max, col_max) {
        if energized_tiles.contains(&(next_pos.0, next_pos.1, beam_direction)) { (*pos, true) } else { (next_pos, false) }
    } else {
        (*pos, true)
    };
}

fn handle_reflector_45_deg_case(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
    energized_tiles: & HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), BeamDirection, bool)
{
    let new_beam_direction = match beam_direction {
        North => East,
        East => North,
        South => West,
        West => South
    };

    return if let Some(next_pos) = checked_advance(pos, new_beam_direction, row_max, col_max) {
        if energized_tiles.contains(&(next_pos.0, next_pos.1, new_beam_direction)) { (*pos, new_beam_direction, true) } else { (next_pos, new_beam_direction, false) }
    } else {
        (*pos, new_beam_direction, true)
    };
}

fn handle_reflector_135_deg_case(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
    energized_tiles: &HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), BeamDirection, bool)
{
    let new_beam_direction = match beam_direction {
        North => West,
        East => South,
        South => East,
        West => North
    };

    return if let Some(next_pos) = checked_advance(pos, new_beam_direction, row_max, col_max) {
        if energized_tiles.contains(&(next_pos.0, next_pos.1, new_beam_direction)) { (*pos, new_beam_direction, true) } else { (next_pos, new_beam_direction, false) }
    } else {
        (*pos, new_beam_direction, true)
    };
}

fn handle_vertical_splitter_case(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
    feature_map: &Vec<Vec<char>>, energized_tiles: &mut HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), bool)
{
    return if beam_direction == North || beam_direction == South {
        handle_empty_case(pos, beam_direction, row_max, col_max, energized_tiles)
    } else {
        if let Some(next_pos) = checked_advance(pos, North, row_max, col_max) {
            trace_beam(&next_pos, North, feature_map, energized_tiles);
        }

        if let Some(next_pos) = checked_advance(pos, South, row_max, col_max) {
            trace_beam(&next_pos, South, feature_map, energized_tiles);
        }

        (*pos, true)
    };
}

fn handle_horizontal_splitter_case(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
    feature_map: &Vec<Vec<char>>, energized_tiles: &mut HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), bool)
{
    return if beam_direction == West || beam_direction == East {
        handle_empty_case(pos, beam_direction, row_max, col_max, energized_tiles)
    } else {
        if let Some(next_pos) = checked_advance(pos, West, row_max, col_max) {
            trace_beam(&next_pos, West, feature_map, energized_tiles);
        }

        if let Some(next_pos) = checked_advance(pos, East, row_max, col_max) {
            trace_beam(&next_pos, East, feature_map, energized_tiles);
        }

        (*pos, true)
    };
}

fn checked_advance(starting_pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize) -> Option<(usize, usize)> {
    return match beam_direction {
        North => if starting_pos.0 > 0 { Some((starting_pos.0 - 1, starting_pos.1)) } else { None },
        East => if starting_pos.1 < col_max { Some((starting_pos.0, starting_pos.1 + 1)) } else { None },
        South => if starting_pos.0 < row_max { Some((starting_pos.0 + 1, starting_pos.1)) } else { None },
        West => if starting_pos.1 > 0 { Some((starting_pos.0, starting_pos.1 - 1)) } else { None }
    }
}

fn debug_print(energized_tiles: &HashSet<(usize, usize, BeamDirection)>, row_max: usize, col_max: usize) {
    for row_index in 0..=row_max {
        for col_index in 0..=col_max {
            let energized = energized_tiles.contains(&(row_index, col_index, North)) ||
                energized_tiles.contains(&(row_index, col_index, East)) ||
                energized_tiles.contains(&(row_index, col_index, South)) ||
                energized_tiles.contains(&(row_index, col_index, West));
            print!("{}", if energized {'#'} else {'.'});
        }
        print!("\n");
    }
}