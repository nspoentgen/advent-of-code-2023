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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum BeamDirection { North, East, South, West }

struct SolverInputs<'a> {
    pub starting_pos: (usize, usize),
    pub beam_direction: BeamDirection,
    pub feature_map: &'a Vec<Vec<char>>,
}

fn main() {
    //Parse data
    let path = Path::new("src/day16_part1/input.txt");
    let feature_map = parse_data(&path);

    //Trace beam on each edge tile, keeping track of best position
    let max_row_index = feature_map.len() - 1;
    let max_col_index = feature_map[0].len() - 1;
    let mut max_initial_conditions = (usize::MAX, usize::MAX, North);
    let mut max_count = 0usize;

    for col_index in 0..feature_map[0].len() {
        let num_energized_tiles = trace_beam_solver((0, col_index), South, &feature_map);
        update_max(&(0, col_index, South), &num_energized_tiles, &mut max_initial_conditions, &mut max_count);
    }

    for row_index in 0..feature_map.len() {
        let num_energized_tiles = trace_beam_solver((row_index, max_col_index), West, &feature_map);
        update_max(&(row_index, max_col_index, West), &num_energized_tiles, &mut max_initial_conditions, &mut max_count);
    }

    for col_index in 0..feature_map[0].len() {
        let num_energized_tiles = trace_beam_solver((max_row_index, col_index), North, &feature_map);
        update_max(&(max_row_index, col_index, North), &num_energized_tiles, &mut max_initial_conditions, &mut max_count);
    }

    for row_index in 0..feature_map.len() {
        let num_energized_tiles = trace_beam_solver((row_index, 0), East, &feature_map);
        update_max(&(row_index, 0, East), &num_energized_tiles, &mut max_initial_conditions, &mut max_count);
    }

    //Print result
    println!("The maximum configuration is [({}, {}, {:?}), {}]", max_initial_conditions.0, max_initial_conditions.1, max_initial_conditions.2, max_count.to_formatted_string(&Locale::en));
}

fn update_max(test_pos: &(usize, usize, BeamDirection), test_count: &usize, max_pos: &mut (usize, usize, BeamDirection), max_count: &mut usize) {
    if test_count > max_count {
        *max_pos = *test_pos;
        *max_count = *test_count;
    }
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

fn trace_beam_solver(starting_pos: (usize, usize), starting_beam_direction: BeamDirection, feature_map: &Vec<Vec<char>>) -> usize {
    let mut energized_tiles = HashSet::<(usize, usize, BeamDirection)>::new();
    let mut work_stack = Vec::<SolverInputs>::new();
    let initial_inputs = SolverInputs {
        starting_pos: starting_pos,
        beam_direction: starting_beam_direction,
        feature_map: &feature_map,
    };
    work_stack.push(initial_inputs);

    while let Some(next_work_item) = work_stack.pop() {
        let additional_work_items = trace_beam(next_work_item, &mut energized_tiles);
        for work_item in additional_work_items.into_iter().rev() {
            work_stack.push(work_item);
        }
    }

    return energized_tiles
        .iter()
        .map(|x| (x.0, x.1))
        .unique()
        .count();
}

fn trace_beam<'a>(mut solver_inputs: SolverInputs<'a>, energized_tiles: &mut HashSet<(usize, usize, BeamDirection)>) -> Vec<SolverInputs<'a>>
{
    let row_max = solver_inputs.feature_map.len() - 1;
    let col_max = solver_inputs.feature_map[0].len() - 1;
    let mut complete = false;
    let mut current_pos = solver_inputs.starting_pos.clone();
    let mut additional_work_items = Vec::<SolverInputs>::new();

    while !complete {
        energized_tiles.insert((current_pos.0, current_pos.1, solver_inputs.beam_direction));

        //debug_print(energized_tiles, row_max, col_max);
        //print!("\n");

        let feature = solver_inputs.feature_map[current_pos.0][current_pos.1];

        if feature == EMPTY {
            (current_pos, complete) = handle_empty_case(&current_pos, solver_inputs.beam_direction, row_max, col_max, energized_tiles);
        } else if feature == REFLECTOR_45_DEG {
            (current_pos, solver_inputs.beam_direction, complete) = handle_reflector_45_deg_case(&current_pos, solver_inputs.beam_direction, row_max, col_max, energized_tiles);
        } else if feature == REFLECTOR_135_DEG {
            (current_pos, solver_inputs.beam_direction, complete) = handle_reflector_135_deg_case(&current_pos, solver_inputs.beam_direction, row_max, col_max, energized_tiles);
        } else if feature == VERTICAL_SPLITTER {
            (current_pos, complete, additional_work_items) = handle_vertical_splitter_case(&current_pos, solver_inputs.beam_direction, row_max, col_max, solver_inputs.feature_map, energized_tiles);
        } else {
            (current_pos, complete, additional_work_items) = handle_horizontal_splitter_case(&current_pos, solver_inputs.beam_direction, row_max, col_max, solver_inputs.feature_map, energized_tiles);
        }
    }

    return additional_work_items;
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

fn handle_vertical_splitter_case<'a>(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
                                     feature_map: &'a Vec<Vec<char>>, energized_tiles: &mut HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), bool, Vec<SolverInputs<'a>>)
{
    let mut additional_work_items = Vec::<SolverInputs>::new();

    return if beam_direction == North || beam_direction == South {
        let result = handle_empty_case(pos, beam_direction, row_max, col_max, energized_tiles);
        (result.0, result.1, additional_work_items)
    } else {
        let advance_result = checked_advance(pos, North, row_max, col_max);
        if advance_result.is_some() && !energized_tiles.contains(&(advance_result.unwrap().0, advance_result.unwrap().1, North)) {
            let sub_problem_inputs = SolverInputs {
                starting_pos: advance_result.unwrap(),
                beam_direction: North,
                feature_map: feature_map,
            };
            additional_work_items.push(sub_problem_inputs);
        }

        let advance_result = checked_advance(pos, South, row_max, col_max);
        if advance_result.is_some() && !energized_tiles.contains(&(advance_result.unwrap().0, advance_result.unwrap().1, South)) {
            let sub_problem_inputs = SolverInputs {
                starting_pos: advance_result.unwrap(),
                beam_direction: South,
                feature_map: feature_map,
            };
            additional_work_items.push(sub_problem_inputs)
        }

        (*pos, true, additional_work_items)
    };
}

fn handle_horizontal_splitter_case<'a>(pos: &(usize, usize), beam_direction: BeamDirection, row_max: usize, col_max: usize,
                                       feature_map: &'a Vec<Vec<char>>, energized_tiles: & mut HashSet<(usize, usize, BeamDirection)>) -> ((usize, usize), bool, Vec<SolverInputs<'a>>)
{
    let mut additional_work_items = Vec::<SolverInputs>::new();

    return if beam_direction == West || beam_direction == East {
        let result = handle_empty_case(pos, beam_direction, row_max, col_max, energized_tiles);
        (result.0, result.1, additional_work_items)
    } else {
        let advance_result = checked_advance(pos, West, row_max, col_max);
        if advance_result.is_some() && !energized_tiles.contains(&(advance_result.unwrap().0, advance_result.unwrap().1, West)) {
            let sub_problem_inputs = SolverInputs {
                starting_pos: advance_result.unwrap(),
                beam_direction: West,
                feature_map: feature_map,
            };
            additional_work_items.push(sub_problem_inputs);
        }

        let advance_result = checked_advance(pos, East, row_max, col_max);
        if advance_result.is_some() && !energized_tiles.contains(&(advance_result.unwrap().0, advance_result.unwrap().1, East)) {
            let sub_problem_inputs = SolverInputs {
                starting_pos: advance_result.unwrap(),
                beam_direction: East,
                feature_map: feature_map,
            };
            additional_work_items.push(sub_problem_inputs);
        }

        (*pos, true, additional_work_items)
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