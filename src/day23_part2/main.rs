use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

const PATH: char = '.';
const FOREST: char = '#';

type WorkItem = ((usize, usize), Vec::<(usize, usize)>);


fn main() {
    let path = Path::new("src/day23_part1/input.txt");
    let map = parse_data(&path);
    let (start_pos, end_pos) = find_terminal_positions(&map);
    let preprocessed_graph = preprocess_graph(&map, &start_pos, &end_pos);
    let max_steps = get_max_steps(&start_pos, &end_pos, &preprocessed_graph);

    println!("The maximum number of steps is {}", max_steps.to_formatted_string(&Locale::en));
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

fn find_terminal_positions(map: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    //Per problem definition, the start and end positions have exactly one path object in each row
    let start_col = map[0].iter().position(|c| *c == PATH).unwrap();
    let end_col = map[map.len() - 1].iter().position(|c| *c == PATH).unwrap();

    return ((0, start_col), (map.len() - 1, end_col));
}



fn get_max_steps(start_pos: &(usize, usize), end_pos: &(usize, usize), preprocessed_graph: &HashMap<((usize, usize), (usize, usize)), usize>) -> usize {
    //DFS
    let mut max_steps = 0usize;
    let mut work_stack = Vec::<WorkItem>::new();
    let initial_visited = Vec::<(usize, usize)>::new();
    work_stack.extend(generate_work_items(start_pos, preprocessed_graph, &initial_visited));

    while work_stack.len() > 0 {
        let mut work_item = work_stack.pop().unwrap();

        if work_item.0 == *end_pos {
            work_item.1.push(work_item.0);

            let mut path_total = 0usize;
            for i in 0..work_item.1.len() - 1 {
                path_total += preprocessed_graph[&(work_item.1[i], work_item.1[i + 1])]
            }

            if path_total > max_steps {
                max_steps = path_total
            }
        } else {
            work_stack.extend(generate_work_items(&work_item.0, preprocessed_graph, &work_item.1))
        }
    }

    return max_steps;
}

fn generate_work_items(current_pos: &(usize, usize), preprocessed_graph: &HashMap<((usize, usize), (usize, usize)), usize>, visited: &Vec<(usize, usize)>) -> Vec<WorkItem> {
    let mut work_items = Vec::<WorkItem>::new();

    for valid_move in get_valid_moves_without_revisit(current_pos, preprocessed_graph, visited) {
        let mut child_visited = visited.clone();
        child_visited.push(current_pos.clone());
        work_items.push((valid_move, child_visited));
    }

    return work_items;
}

fn preprocess_graph(map: &Vec<Vec<char>>, start_pos: &(usize, usize), end_pos: &(usize, usize)) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut preprocess_graph = HashMap::<((usize, usize), (usize, usize)), usize>::new();
    let junction_tiles = find_junction_tiles(map, start_pos, end_pos);

    for tile in &junction_tiles {
        update_preprocessed_graph(tile, &junction_tiles, map, &mut preprocess_graph);
    }

    return preprocess_graph;
}

fn find_junction_tiles(map: &Vec<Vec<char>>, start_pos: &(usize, usize), end_pos: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut junction_tiles = HashSet::<(usize, usize)>::new();

    for row in 0..map.iter().len() {
        for col in 0..map[row].iter().len() {
            if map[row][col] != FOREST && (
                (row, col) == *start_pos ||
                (row, col) == *end_pos ||
                get_valid_moves(&(row, col), &None, map).len() > 2) {

                junction_tiles.insert((row, col));
            }
        }
    }

    return junction_tiles;
}

fn update_preprocessed_graph(tile_position: &(usize, usize), junction_tiles: &HashSet<(usize, usize)>, map: &Vec<Vec<char>>, path_lengths_map: &mut HashMap<((usize, usize), (usize, usize)), usize>) {
    for first_step_position in get_valid_moves(tile_position, &None, map) {
        let mut continue_walking_path = true;
        let mut previous_step = *tile_position;
        let mut current_step = first_step_position;
        let mut step_count = 1usize;

        while continue_walking_path {
            if junction_tiles.contains(&current_step) {
                if path_lengths_map.contains_key(&(*tile_position, current_step)) {
                    if step_count > path_lengths_map[&(*tile_position, current_step)] {
                        path_lengths_map.insert((*tile_position, current_step), step_count);
                    }
                } else {
                    path_lengths_map.insert((*tile_position, current_step), step_count);
                }

                continue_walking_path = false;
            } else {
                let valid_moves = get_valid_moves(&current_step, &Some(previous_step), map);

                match valid_moves.len() {
                    0 => continue_walking_path = false,
                    1 => {
                        if valid_moves[0] == first_step_position {
                            continue_walking_path  = false;
                        } else {
                            previous_step = current_step;
                            current_step = valid_moves[0];
                        }
                    },
                    _ => panic!("Logic error. This state should not be possible.")
                }
            }

            step_count += 1;
        }
    }
}

fn get_valid_moves(position: &(usize, usize), previous_position: &Option<(usize, usize)>, map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let signed_position = (position.0 as isize, position.1 as isize);
    let mut valid_positions = Vec::<(usize, usize)>::new();

    for delta in deltas {
        let test_position = (signed_position.0 + delta.0, signed_position.1 + delta.1);
        if test_position.0 >= 0 && test_position.0 < map.len() as isize && test_position.1 >= 0 &&
            test_position.1 < map[0].len() as isize && map[test_position.0 as usize][test_position.1 as usize] != FOREST &&
            !(previous_position.is_some() && test_position.0 == previous_position.unwrap().0 as isize && test_position.1 == previous_position.unwrap().1 as isize)
        {
            valid_positions.push( (test_position.0 as usize, test_position.1 as usize));
        }
    }

    return valid_positions;
}

fn get_valid_moves_without_revisit(position: &(usize, usize), preprocessed_graph: &HashMap<((usize, usize), (usize, usize)), usize>, visited: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    return preprocessed_graph
        .iter()
        .filter(|entry| entry.0.0 == *position)
        .map(|entry| entry.0.1)
        .filter(|dest| !visited.contains(dest))
        .collect_vec();
}
