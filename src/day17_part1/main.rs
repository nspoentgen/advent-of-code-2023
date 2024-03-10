use std::fs::File;
use std::hash::Hash;
use std::collections::{HashSet, HashMap};
use std::io::{BufReader, BufRead};
use std::path::Path;
use rayon::prelude::*;
use num_format::{Locale, ToFormattedString};
use Direction::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct NodeState {
    pub row_index: usize,
    pub col_index: usize,
    pub direction: Direction,
    pub straights_left: u32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction { North, East, South, West }

const MAX_CONSECUTIVE_STRAIGHTS: u32 = 3;

fn main () {
    //Parse data
    let path = Path::new("src/day17_part1/input.txt");
    let heat_loss_reference = parse_data(&path);

    //Generate data for Dijkstra's algorithm
    let (graph, adjacent_node_indices, node_index_map) = generate_graph(&heat_loss_reference);

    //Find minimum
    let minimum_heat_loss = generate_possible_end_states(heat_loss_reference.len() - 1, heat_loss_reference[0].len() - 1)
        .par_iter()
        .map(|goal_state| find_minimum_heat_loss(&graph, &adjacent_node_indices, &heat_loss_reference, node_index_map[&goal_state]))
        .filter_map(|x| x)
        .min();

    //Print answer
    println!("The minimum heat loss is {}", minimum_heat_loss.unwrap().to_formatted_string(&Locale::en))
}

fn parse_data(path: &Path) -> Vec<Vec<usize>> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>())
        .collect();
}

fn generate_graph(heat_loss_reference: &Vec<Vec<usize>>) -> (Vec<NodeState>, Vec<Vec<usize>>, HashMap<NodeState, usize>) {
    let mut node_index_map = HashMap::<NodeState, usize>::new();
    let mut graph = Vec::<NodeState>::new();
    let mut node_index = 0usize;
    let all_directions_vec = vec![North, East, South, West];

    //Special case for starting node. Assumed convention to place it at the front of the graph
    let initial_state = NodeState {
        row_index: 0,
        col_index: 0,
        direction: East,
        straights_left: MAX_CONSECUTIVE_STRAIGHTS,
    };
    node_index_map.insert(initial_state.clone(), node_index);
    graph.push(initial_state);
    node_index += 1;

    //All other nodes
    for row_index in 0..heat_loss_reference.len() {
        for col_index in 0..heat_loss_reference[row_index].len() {
            for direction in &all_directions_vec {
                for straights_left in 0..MAX_CONSECUTIVE_STRAIGHTS {
                    let node = NodeState {
                        row_index,
                        col_index,
                        direction: *direction,
                        straights_left,
                    };

                    node_index_map.insert(node.clone(), node_index);
                    graph.push(node);
                    node_index += 1;
                }
            }
        }
    }

    //Populate adjacent nodes
    let row_max = heat_loss_reference.len() - 1;
    let col_max = heat_loss_reference[0].len() - 1;
    let mut adjacent_node_indices: Vec<Vec<usize>> = vec![Vec::<usize>::new(); graph.len()];

    for index in 0..graph.len() {
        adjacent_node_indices[index] = get_valid_nodes(&graph[index], row_max, col_max)
            .iter()
            .map(|x| node_index_map[x])
            .collect::<Vec<usize>>();
    }

    return (graph, adjacent_node_indices, node_index_map);
}

fn get_valid_nodes(current_state: &NodeState, row_max: usize, col_max: usize) -> Vec<NodeState> {
    let mut valid_states = Vec::<NodeState>::new();
    
    //Conditionally add left move
    if let Some(left_move) = generate_left_move(&current_state, row_max, col_max) {
        valid_states.push(left_move);
    }

    //Conditionally add right move
    if let Some(right_move) = generate_right_move(&current_state, row_max, col_max) {
        valid_states.push(right_move);
    }

    //Conditionally add straight move
    if let Some(straight_move) = generate_straight_move(&current_state, row_max, col_max) {
        valid_states.push(straight_move);
    }
    
    return valid_states;
}

fn generate_left_move(current_state: &NodeState, row_max: usize, col_max: usize) -> Option<NodeState> {
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
            straights_left: MAX_CONSECUTIVE_STRAIGHTS - 1
        };

        left_move_state = Some(possible_state);
    }

    return left_move_state;
}

fn generate_right_move(current_state: &NodeState, row_max: usize, col_max: usize) -> Option<NodeState> {
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
            straights_left: MAX_CONSECUTIVE_STRAIGHTS - 1
        };

        right_move_state = Some(possible_state);
    }

    return right_move_state;
}

fn generate_straight_move(current_state: &NodeState, row_max: usize, col_max: usize) -> Option<NodeState> {
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

            straight_move_state = Some(possible_state);
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

fn generate_possible_end_states(row_max: usize, col_max: usize) -> Vec<NodeState> {
    let mut possible_end_states = Vec::<NodeState>::new();
    let possible_directions = vec![East, South];

    for straights_left in 0..MAX_CONSECUTIVE_STRAIGHTS {
        for direction in &possible_directions {
            possible_end_states.push(NodeState {
                row_index: row_max,
                col_index: col_max,
                direction: *direction,
                straights_left
            });
        }
    }

    return possible_end_states;
}

fn find_minimum_heat_loss(graph: &Vec<NodeState>, adjacent_node_indices: &Vec<Vec<usize>>, heat_loss_reference: &Vec<Vec<usize>>, goal_node_index: usize) -> Option<usize> {
    let mut heat_loss = vec![usize::MAX; graph.len()];
    let mut previous: Vec<Option<usize>> = vec![None; graph.len()];
    let mut search_set = HashSet::<usize>::new();

    for node_index in 0..graph.len() {
        heat_loss[node_index] = usize::MAX;
        previous[node_index] = None;
        search_set.insert(node_index);
    }

    heat_loss[0] = 0;

    while !&search_set.is_empty() {
        let mut min_heat_loss = usize::MAX;
        let mut min_heat_loss_node_index = 0usize;

        for node_index in &search_set {
            if heat_loss[*node_index] < min_heat_loss {
                min_heat_loss_node_index = *node_index;
                min_heat_loss = heat_loss[*node_index];
            }
        }

        if min_heat_loss_node_index == goal_node_index {
            return Some(min_heat_loss);
        }
        search_set.remove(&min_heat_loss_node_index);

        for adjacent_node_index in &adjacent_node_indices[min_heat_loss_node_index] {
            let alternate = heat_loss[min_heat_loss_node_index] + heat_loss_reference[graph[*adjacent_node_index].row_index][graph[*adjacent_node_index].col_index];
            if alternate < heat_loss[*adjacent_node_index] {
                heat_loss[*adjacent_node_index] = alternate;
            }
        }
    }

    return None;
}