use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

const START: char = 'S';
const ROCK: char = '#';
const GARDEN_PLOT: char = '.';
const MAX_NUM_STEPS: u64 = 64;

//Convention: x-axis positive right, y-axis positive down
type Coordinate = (usize, usize);

#[derive(Eq, PartialEq, Hash, Clone)]
struct WalkState {
    pub position: Coordinate,
    pub steps_left: u64,
}

fn main() {
    //Parse map
    let path = Path::new("src/day21_part1/input.txt");
    let mut map = parse_data(&path);

    //Record starting position. Per problem statement, this is a garden
    //spot, so update to make our lives easier later on.
    let starting_position = find_starting_position(&map);
    map[starting_position.0][starting_position.1] = GARDEN_PLOT;

    //Solve using dynamic programming
    let num_positions = get_possible_num_positions(starting_position, &map);
    println!("Num positions = {}", num_positions.to_formatted_string(&Locale::en));
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

fn find_starting_position(data: &Vec<Vec<char>>) -> Coordinate {
    for row_index in 0..data.len() {
        for col_index in 0..data[row_index].len() {
            if data[row_index][col_index] == START {
                return (row_index, col_index);
            }
        }
    }

    panic!("Couldn't find starting position");
}

fn get_possible_num_positions(initial_position: Coordinate, map: &Vec<Vec<char>>) -> usize {
    let initial_state = WalkState { position: initial_position, steps_left: MAX_NUM_STEPS };
    let mut final_positions = HashSet::<Coordinate>::new();
    let mut solved_states = HashSet::<WalkState>::new();

    let mut work_stack = Vec::<WalkState>::new();
    work_stack.push(initial_state.clone());

    while work_stack.len() > 0 {
        let next_work_item = work_stack.pop().unwrap();
        let additional_work_items = walk(next_work_item, &mut final_positions, &mut solved_states, map);
        work_stack.extend(additional_work_items.into_iter().rev());
    }

    return final_positions.len();
}

fn walk(state: WalkState, final_positions: &mut HashSet<Coordinate>, solved_states: &mut HashSet<WalkState>, map: &Vec<Vec<char>>) -> Vec<WalkState> {
    let mut queued_work_items = vec![];

    if state.steps_left == 0 {
        final_positions.insert(state.position);
        solved_states.insert(state.clone());
    } else {
        let next_positions = get_next_positions(state.position, map);
        for next_state in next_positions.iter().map(|p| WalkState {position: *p, steps_left: state.steps_left - 1 }) {
            if !solved_states.contains(&next_state) {
                queued_work_items.push(next_state);
            }
        }

        if queued_work_items.len() > 0 {
            queued_work_items.push(state); //don't forgot to re-add ourselves if we are not done
        } else {
            solved_states.insert(state.clone());
        }
    }

    return queued_work_items;
}

fn get_next_positions(current_position: Coordinate, map: &Vec<Vec<char>>) -> Vec<Coordinate> {
    let current_position_signed = to_signed_coordinate(current_position);
    let possible_positions = [
        (current_position_signed.0 + 1, current_position_signed.1),
        (current_position_signed.0 - 1, current_position_signed.1),
        (current_position_signed.0, current_position_signed.1 + 1),
        (current_position_signed.0, current_position_signed.1 - 1),
    ];

    return possible_positions
        .iter()
        .filter(|&p| p.0 >= 0 && p.1 >= 0 && p.0 < map.len() as isize && p.1 < map[0].len() as isize)
        .map(|&p| to_unsigned_coordinate(p))
        .filter(|&p| map[p.0][p.1] != ROCK)
        .collect_vec();
}

fn to_signed_coordinate(unsigned_coordinate: (usize, usize)) -> (isize, isize) {
    return (unsigned_coordinate.0 as isize, unsigned_coordinate.1 as isize);
}

fn to_unsigned_coordinate(signed_coordinate: (isize, isize)) -> (usize, usize) {
    return (signed_coordinate.0 as usize, signed_coordinate.1 as usize);
}
