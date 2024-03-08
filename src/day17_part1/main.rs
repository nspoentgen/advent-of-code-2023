use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, BufRead};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use num_format::Locale::en;
use rayon::iter::Fold;
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
    let minimum_heat_loss = minimum_heat_loss_solver((0,0), East, &heat_loss_reference);

    //Print answer
    println!("The minimum heat loss is {}", minimum_heat_loss.to_formatted_string(&en))
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

fn minimum_heat_loss_solver(initial_pos: (usize, usize), initial_direction: Direction, heat_loss_reference: &Vec<Vec<u32>>) -> u32 {
    #[derive(Debug)]
    enum Action {
        Process(NodeState),
        Fold(u32)
    }

    let initial_state = NodeState {
        row_index: initial_pos.0,
        col_index: initial_pos.1,
        direction: initial_direction,
        straights_left: 3
    };
    let mut minimum_heat_loss_cache = HashMap::<NodeState, Option<u32>>::new();
    let mut work_stack = Vec::<Action>::new();
    let mut state_stack = Vec::<Vec<u32>>::new();
    let mut state_tracker = Vec::<u32>::new();
    work_stack.push(Action::Fold(0));
    work_stack.push(Action::Process(initial_state));

    while let Some(next_work_item) = work_stack.pop() {
        let work_stack_count = work_stack.iter().count();
        //println!("Work stack = {:?}\nstate stack = {:?}\ncurrent state = {:?}\n", work_stack, state_stack, next_work_item);
        /*
        if work_stack_count > 100 {
            panic!("Debug stop")
        }

         */

        match next_work_item {
            Action::Process(test_state) => {
                let state_trace = work_stack
                    .iter()
                    .filter_map(|x| match x {
                        Action::Process(state) => Some(state),
                        Action::Fold(_) => None
                    })
                    .collect::<HashSet<&NodeState>>();

                let (heat_loss_option, additional_work_items) = get_minimum_heat_loss(&test_state, state_trace, &mut minimum_heat_loss_cache, &heat_loss_reference);

                if let Some(heat_loss) = heat_loss_option {
                    state_tracker.push(heat_loss);
                } else if additional_work_items.iter().count() > 0 {
                    let fold_value = if (test_state.row_index, test_state.col_index) == (0, 0) { 0 } else { heat_loss_reference[test_state.row_index][test_state.col_index] };
                    work_stack.push(Action::Fold(fold_value));
                    state_stack.push(state_tracker.clone());
                    state_tracker.clear();

                    for work_item in additional_work_items.into_iter().rev() {
                        work_stack.push(Action::Process(work_item));
                    }
                }
            },
            Action::Fold(acc) => {
                let mut cumulative_heat_loss = acc;
                if state_tracker.iter().count() > 0 {
                    cumulative_heat_loss += state_tracker.iter().min().unwrap();
                }

                if let Some(next_state_tracker) = state_stack.pop() {
                    state_tracker = next_state_tracker;
                }

                state_tracker.push(cumulative_heat_loss)
            }
        };
    }

    return state_tracker[0];
}

fn get_minimum_heat_loss(current_state: &NodeState, stack_trace: HashSet<&NodeState>, minimum_heat_loss_cache: &mut HashMap<NodeState, Option<u32>>, heat_loss_reference: &Vec<Vec<u32>>) -> (Option<u32>, Vec<NodeState>) {
    let mut min_heat_loss = None;
    let mut update_cache = true;
    let mut additional_work_items = Vec::<NodeState>::new();

    if let Some(cached_result) = minimum_heat_loss_cache.get(current_state) {
        min_heat_loss = *cached_result;
        update_cache = false;

        //println!("Cache hit!");
    } else if current_state.row_index == heat_loss_reference.len() - 1 && current_state.col_index == heat_loss_reference[0].len() - 1 {
        min_heat_loss = Some(*heat_loss_reference.last().unwrap().last().unwrap());
        println!("Reached final node");
    } else {
        additional_work_items = get_next_valid_states(current_state, &stack_trace, heat_loss_reference.len() - 1, heat_loss_reference[0].len() - 1);
    }

    if update_cache {
        minimum_heat_loss_cache.insert(current_state.clone(), min_heat_loss);
    }
    return (min_heat_loss, additional_work_items);
}

fn get_next_valid_states(current_state: &NodeState, stack_trace: &HashSet<&NodeState>, row_max: usize, col_max: usize) -> Vec<NodeState> {
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

fn generate_left_move(current_state: &NodeState, stack_trace: &HashSet<&NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
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

        if stack_trace.iter().all(|&x| (possible_state.row_index, possible_state.col_index) != (x.row_index, x.col_index)) {
            left_move_state = Some(possible_state);
        }
    }

    return left_move_state;
}

fn generate_right_move(current_state: &NodeState, stack_trace: &HashSet<&NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
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

        if stack_trace.iter().all(|&x| (possible_state.row_index, possible_state.col_index) != (x.row_index, x.col_index)) {
            right_move_state = Some(possible_state);
        }
    }

    return right_move_state;
}

fn generate_straight_move(current_state: &NodeState, stack_trace: &HashSet<&NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
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

            if stack_trace.iter().all(|&x| (possible_state.row_index, possible_state.col_index) != (x.row_index, x.col_index)) {
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