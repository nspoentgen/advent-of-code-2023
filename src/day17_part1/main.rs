use std::cmp::Ordering;
use std::fs::File;
use std::hash::Hash;
use std::collections::{HashMap, BinaryHeap};
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapState {
    cost: usize,
    node_index: usize,
}

// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare node indices. This step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.node_index.cmp(&other.node_index))
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
        adjacent_node_indices[index] = get_adjacent_nodes(&graph[index], row_max, col_max)
            .iter()
            .map(|x| node_index_map[x])
            .collect::<Vec<usize>>();
    }

    return (graph, adjacent_node_indices, node_index_map);
}

fn get_adjacent_nodes(current_state: &NodeState, row_max: usize, col_max: usize) -> Vec<NodeState> {
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

    if is_in_bounds(left_move_pos, row_max, col_max) {
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

    if is_in_bounds(right_move_pos, row_max, col_max) {
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
        if is_in_bounds(straight_move_pos, row_max, col_max) {
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

fn is_in_bounds(test_pos: (isize, isize), row_max: usize, col_max: usize) -> bool {
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
    //heat_loss[node_index] = current shortest distance from `start` to `node`
    let mut heat_loss: Vec<_> = (0..adjacent_node_indices.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    //Initial state. Graph is configured such that the initial state is index 0.
    heat_loss[0] = 0;
    heap.push(HeapState { cost: 0, node_index: 0 });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(HeapState { cost, node_index }) = heap.pop() {
        //Return as soon as we have found our goal node
        if node_index == goal_node_index {
            return Some(cost);
        }

        //Continue to the next iteration if we have already exceeded the optimal
        //heat loss. This saves a lot of computation.
        if cost > heat_loss[node_index] {
            continue;
        }

        //For each node we can reach, see if we can find a way with
        //a lower cost going through this node
        for adjacent_node_index in &adjacent_node_indices[node_index] {
            let movement_cost = heat_loss_reference[graph[*adjacent_node_index].row_index][graph[*adjacent_node_index].col_index];
            let next = HeapState { cost: cost + movement_cost, node_index: *adjacent_node_index };

            // If so, add it to the frontier and continue
            if next.cost < heat_loss[*adjacent_node_index] {
                heap.push(next);

                // Relaxation, we have now found a better way
                heat_loss[*adjacent_node_index] = next.cost;
            }
        }
    }

    // Goal not reachable
    return None
}