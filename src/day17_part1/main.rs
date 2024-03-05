use std::collections::HashSet;
use std::hash::Hash;
use Direction::*;

#[derive(Eq, PartialEq, Hash)]
struct NodeState {
    pub row_index: usize,
    pub col_index: usize,
    pub direction: Direction,
    pub straights_left: u32
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction { North, East, South, West }

const MAX_CONSECUTIVE_STRAIGHTS: u32 = 3;

fn main () {

}

fn get_minimum_heat_loss(current_state: &NodeState, visited_states: &mut HashSet<NodeState>, heat_loss_map: &Vec<Vec<u32>>) -> Option<u32> {
    let min_heat_loss = None;

    for next_state in get_next_valid_states(current_state, visited_states, heat_loss_map.len() - 1, heat_loss_map[0].len() - 1) {

    }
}

fn get_next_valid_states(current_state: &NodeState, visited_states: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Vec<NodeState> {
    let mut valid_states = Vec::<NodeState>::new();
    
    //Conditionally add left move
    if let Some(left_move) = generate_left_move(&current_state, &visited_states, row_max, col_max) {
        valid_states.push(left_move);
    }

    //Conditionally add right move
    if let Some(right_move) = generate_right_move(&current_state, &visited_states, row_max, col_max) {
        valid_states.push(right_move);
    }

    //Conditionally add straight move
    if let Some(straight_move) = generate_straight_move(&current_state, &visited_states, row_max, col_max) {
        valid_states.push(straight_move);
    }
    
    return valid_states;
}

fn generate_left_move(current_state: &NodeState, visited_states: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
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

        if !visited_states.contains(&possible_state) {
            left_move_state = Some(possible_state);
        }
    }

    return left_move_state;
}

fn generate_right_move(current_state: &NodeState, visited_states: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
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

        if !visited_states.contains(&possible_state) {
            right_move_state = Some(possible_state);
        }
    }

    return right_move_state;
}

fn generate_straight_move(current_state: &NodeState, visited_states: &HashSet<NodeState>, row_max: usize, col_max: usize) -> Option<NodeState> {
    let mut straight_move_state = None;

    let straight_move_pos = advance_pos(current_state.row_index as isize, current_state.col_index as isize, current_state.direction);

    if is_valid_state(straight_move_pos, row_max, col_max) && current_state.straights_left > 0 {
        let possible_state = NodeState {
            row_index: straight_move_pos.0 as usize,
            col_index: straight_move_pos.1 as usize,
            direction: current_state.direction,
            straights_left: current_state.straights_left - 1
        };

        if !visited_states.contains(&possible_state) {
            straight_move_state = Some(possible_state);
        }
    }

    return straight_move_state;
}

fn advance_pos(row_index: isize, col_index:isize, direction: Direction) -> (isize, isize) {
    return match direction {
        North => (row_index as isize - 1, col_index as isize),
        East => (row_index as isize, col_index as isize + 1),
        South => (row_index as isize + 1, col_index as isize),
        West => (row_index, col_index as isize - 1)
    };
}

fn is_valid_state(test_pos: (isize, isize), row_max: usize, col_max: usize) -> bool {
    return test_pos.0 >= 0 &&
        test_pos.0 <= row_max as isize &&
        test_pos.1 >= 0 &&
        test_pos.1 <= col_max as isize;
}