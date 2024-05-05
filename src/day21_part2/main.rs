use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

const START: char = 'S';
const ROCK: char = '#';
const GARDEN_PLOT: char = '.';
const MAX_NUM_STEPS: u64 = 500;

//Convention: x-axis positive right, y-axis positive down
type UnsignedCoordinate = (usize, usize);
type SignedCoordinate = (isize, isize);

#[derive(Eq, PartialEq, Hash, Clone)]
struct WalkState {
    pub map_position: SignedCoordinate,
    pub tile_position: UnsignedCoordinate,
    pub steps_left: u64,
}

fn main() {
    //Parse map
    let path = Path::new("src/day21_part1/test_input.txt");
    let mut map = parse_data(&path);

    //Record starting tile_position. Per problem statement, this is a garden
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

fn find_starting_position(data: &Vec<Vec<char>>) -> UnsignedCoordinate {
    for row_index in 0..data.len() {
        for col_index in 0..data[row_index].len() {
            if data[row_index][col_index] == START {
                return (row_index, col_index);
            }
        }
    }

    panic!("Couldn't find starting tile_position");
}

fn get_possible_num_positions(initial_position: UnsignedCoordinate, map: &Vec<Vec<char>>) -> usize {
    let initial_state = WalkState {map_position: to_signed_coordinate(initial_position),  tile_position: initial_position, steps_left: MAX_NUM_STEPS };
    let initial_visited = HashSet::<SignedCoordinate>::new();
    let mut final_positions = HashSet::<SignedCoordinate>::new();
    let mut solved_states = HashSet::<WalkState>::new();

    let mut work_queue = Vec::<(WalkState, HashSet<SignedCoordinate>)>::new();
    work_queue.push((initial_state.clone(), initial_visited));

    while work_queue.len() > 0 {
        let next_work_item = work_queue.pop().unwrap();
        let additional_work_items = walk(next_work_item.0, next_work_item.1, &mut final_positions, &mut solved_states, map);
        work_queue.extend(additional_work_items.into_iter().rev());
    }

    return final_positions.len();
}

fn walk(state: WalkState, visited: HashSet<SignedCoordinate>, final_positions: &mut HashSet<SignedCoordinate>, solved_states: &mut HashSet<WalkState>, map: &Vec<Vec<char>>) -> Vec<(WalkState, HashSet<SignedCoordinate>)> {
    let mut queued_work_items = vec![];

    if state.steps_left == 0 {
        final_positions.insert(state.map_position);
        solved_states.insert(state.clone());
    } else {
        let next_positions = get_next_positions(state.map_position, state.tile_position, &visited, map);
        for next_state in next_positions.iter().map(|p| WalkState { map_position: p.0, tile_position: p.1, steps_left: state.steps_left - 1 }) {
            if !solved_states.contains(&next_state) {
                queued_work_items.push((next_state, visited.clone()));
            }
        }

        if queued_work_items.len() > 0 {
            queued_work_items.push((state, visited.clone())); //don't forgot to re-add ourselves if we are not done
        } else {
            solved_states.insert(state.clone());
        }
    }

    return queued_work_items;
}

fn get_next_positions(current_map_position: SignedCoordinate, current_tile_position: UnsignedCoordinate,
                      visited: &HashSet<SignedCoordinate>, map: &Vec<Vec<char>>) -> Vec<(SignedCoordinate, UnsignedCoordinate)> {
    let get_tile_value = |map_value: usize, delta: isize, is_row: bool| -> usize {
        let base = if is_row { map.len() as isize } else { map[0].len() as isize };
        return (((map_value as isize) + base + delta) % base) as usize;
    };

    let possible_positions = [
        ((current_map_position.0 + 1, current_map_position.1), (get_tile_value(current_tile_position.0, 1, true), current_tile_position.1)),
        ((current_map_position.0 - 1, current_map_position.1), (get_tile_value(current_tile_position.0, -1, true), current_tile_position.1)),
        ((current_map_position.0, current_map_position.1 + 1), (current_tile_position.0, get_tile_value(current_tile_position.1, 1, false))),
        ((current_map_position.0, current_map_position.1 - 1), (current_tile_position.0, get_tile_value(current_tile_position.1, -1, false)))
    ];

    return possible_positions
        .into_iter()
        .filter(|&p| !visited.contains(&p.0))
        .filter(|&p| map[p.1.0][p.1.1] != ROCK)
        .collect_vec();
}

fn to_signed_coordinate(unsigned_coordinate: (usize, usize)) -> (isize, isize) {
    return (unsigned_coordinate.0 as isize, unsigned_coordinate.1 as isize);
}
