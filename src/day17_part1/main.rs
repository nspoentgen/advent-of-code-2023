use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use Direction::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct NodeState {
    pub row_index: usize,
    pub col_index: usize,
    pub direction: Direction,
    pub straights_left: u32
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug

)]
enum Direction { North, East, South, West }

const MAX_CONSECUTIVE_STRAIGHTS: u32 = 3;

fn main () {
    //Parse data
    let path = Path::new("src/day17_part1/input.txt");
    let heat_loss_reference = parse_data(&path);

    //Get minimum using dynamic programming
    let initial_state = NodeState {
        row_index: 0,
        col_index: 0,
        direction: East,
        straights_left: 3
    };
    let mut minimum_heat_loss_cache = HashMap::<NodeState, Option<u32>>::new();
    let stack_trace = HashSet::<NodeState>::new();
    let minimum_heat_loss = get_minimum_heat_loss(&initial_state, &mut minimum_heat_loss_cache, &heat_loss_reference, stack_trace).unwrap();

    //Print answer
    println!("The minimum heat loss is {}", minimum_heat_loss.to_formatted_string(&Locale::en))
}

fn parse_data(path: &Path) -> Vec<Vec<u32>> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();
}

fn get_minimum_heat_loss(current_state: &NodeState, minimum_heat_loss_cache: &mut HashMap<NodeState, Option<u32>>, heat_loss_reference: &Vec<Vec<u32>>, mut stack_trace: HashSet<NodeState>) -> Option<u32> {
    println!("Node = {:?}), stack depth = {}", current_state, stack_trace.iter().count());
    stack_trace.insert(current_state.clone());
    let mut min_heat_loss = None;
    let mut update_cache = true;

    if let Some(cached_result) = minimum_heat_loss_cache.get(current_state) {
        min_heat_loss = *cached_result;
        update_cache = false;
    } else if current_state.row_index == heat_loss_reference.len() - 1 && current_state.col_index == heat_loss_reference[0].len() - 1 {
        min_heat_loss = Some(*heat_loss_reference.last().unwrap().last().unwrap());
        println!("Reached final node");
    } else {
        let mut sub_problem_min_heat_loss_option = None;

        for next_state in get_next_valid_states(current_state, &stack_trace, heat_loss_reference.len() - 1, heat_loss_reference[0].len() - 1) {
            if let Some(sub_problem_min_heat_loss) = get_minimum_heat_loss(&next_state, minimum_heat_loss_cache, heat_loss_reference, stack_trace.clone()) {
                if sub_problem_min_heat_loss_option.is_none() || sub_problem_min_heat_loss < sub_problem_min_heat_loss_option.unwrap() {
                    sub_problem_min_heat_loss_option = Some(sub_problem_min_heat_loss);
                }
            }
        }

        if let Some(sub_problem_min_heat_loss) = sub_problem_min_heat_loss_option {
            min_heat_loss = Some(sub_problem_min_heat_loss + heat_loss_reference[current_state.row_index][current_state.col_index]);
        }
    }

    if update_cache {
        minimum_heat_loss_cache.insert(current_state.clone(), min_heat_loss);
    }
    return min_heat_loss;
}

fn get_next_valid_states(current_state: &NodeState, stack_trace: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Vec<NodeState> {
    let mut valid_states = Vec::<NodeState>::new();
    
    //Conditionally add left move
    if let Some(left_move) = generate_left_move(&current_state, stack_trace, row_max, col_max) {
        valid_states.push(left_move);
    }

    //Conditionally add right move
    if let Some(right_move) = generate_right_move(&current_state, stack_trace, row_max, col_max) {
        valid_states.push(right_move);
    }

    //Conditionally add straight move
    if let Some(straight_move) = generate_straight_move(&current_state, stack_trace, row_max, col_max) {
        valid_states.push(straight_move);
    }
    
    return valid_states;
}

fn generate_left_move(current_state: &NodeState, stack_trace: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
    let mut left_move_state = None;

    let left_move_direction: Direction = match current_state.direction {
        North => West,
        East => North,
        South => East,
        West => South
    };

    let left_move_pos = advance_pos(current_state.row_index as isize, current_state.col_index as isize, left_move_direction);

    if is_valid_state(left_move_pos, row_max, col_max) {
        let possible_state = NodeState {
            row_index: left_move_pos.0 as usize,
            col_index: left_move_pos.1 as usize,
            direction: left_move_direction,
            straights_left: MAX_CONSECUTIVE_STRAIGHTS
        };
        if !stack_trace.contains(&possible_state) {
            left_move_state = Some(possible_state);
        }
    }

    return left_move_state;
}

fn generate_right_move(current_state: &NodeState, stack_trace: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
    let mut right_move_state = None;

    let right_move_direction: Direction = match current_state.direction {
        North => East,
        East => South,
        South => West,
        West => North
    };

    let right_move_pos = advance_pos(current_state.row_index as isize, current_state.col_index as isize, right_move_direction);

    if is_valid_state(right_move_pos, row_max, col_max) {
        let possible_state = NodeState {
            row_index: right_move_pos.0 as usize,
            col_index: right_move_pos.1 as usize,
            direction: right_move_direction,
            straights_left: MAX_CONSECUTIVE_STRAIGHTS
        };
        if !stack_trace.contains(&possible_state) {
            right_move_state = Some(possible_state);
        }
    }

    return right_move_state;
}

fn generate_straight_move(current_state: &NodeState, stack_trace: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
    let mut straight_move_state = None;

    if current_state.straights_left > 0 {
        let straight_move_pos = advance_pos(current_state.row_index as isize, current_state.col_index as isize, current_state.direction);
        if is_valid_state(straight_move_pos, row_max, col_max) {
            let possible_state = NodeState {
                row_index: straight_move_pos.0 as usize,
                col_index: straight_move_pos.1 as usize,
                direction: current_state.direction,
                straights_left: current_state.straights_left - 1,
            };
            if !stack_trace.contains(&possible_state) {
                straight_move_state = Some(possible_state);
            }
        }
    }

    return straight_move_state;
}

fn advance_pos(row_index: isize, col_index:isize, direction: Direction) -> (isize, isize) {
    return match direction {
        North => (row_index - 1, col_index),
        East => (row_index, col_index + 1),
        South => (row_index + 1, col_index),
        West => (row_index, col_index - 1)
    };
}

fn is_valid_state(test_pos: (isize, isize), row_max: usize, col_max: usize) -> bool {
    return test_pos.0 >= 0 &&
        test_pos.0 <= row_max as isize &&
        test_pos.1 >= 0 &&
        test_pos.1 <= col_max as isize;
}