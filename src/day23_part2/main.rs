use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use Direction::*;

const PATH: char = '.';
const FOREST: char = '#';

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction { North, East, South, West }

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct CacheKey {
    pub position: (usize, usize),
    pub incoming_direction: Direction,
}


fn main() {
    let path = Path::new("src/day23_part1/test_input.txt");
    let map = parse_data(&path);
    let (start_pos, end_pos) = find_terminal_positions(&map);
    let max_steps = get_max_steps(&start_pos, &end_pos, &map);

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

fn get_valid_moves(position: &(usize, usize), map: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) -> Vec<CacheKey> {
    let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let signed_position = (position.0 as isize, position.1 as isize);
    let mut valid_positions = Vec::<CacheKey>::new();

    for delta in deltas {
        let test_position = (signed_position.0 + delta.0, signed_position.1 + delta.1);
        if test_position.0 >= 0 && test_position.0 < map.len() as isize && test_position.1 >= 0 &&
            test_position.1 < map[0].len() as isize && map[test_position.0 as usize][test_position.1 as usize] != FOREST &&
            !path.contains(&(test_position.0 as usize, test_position.1 as usize))
        {
            let unsigned_test_position = (test_position.0 as usize, test_position.1 as usize);
            let direction = get_direction(&position, &unsigned_test_position);
            valid_positions.push(CacheKey{ position: unsigned_test_position, incoming_direction: direction});
        }
    }

    return valid_positions;
}

fn get_direction(current_pos: &(usize, usize), next_pos: &(usize, usize)) -> Direction {
    return if next_pos.0 < current_pos.0  {
        North
    } else if next_pos.1 > current_pos.1 {
        East
    } else if next_pos.0 > current_pos.0 {
        South
    } else {
        West
    }
}

fn get_max_steps(start_pos: &(usize, usize), end_pos: &(usize, usize), map: &Vec<Vec<char>>) -> usize {
    let start_cache_key: CacheKey = CacheKey{position: *start_pos, incoming_direction: South};
    let end_cache_key: CacheKey = CacheKey{position: *end_pos, incoming_direction: South};

    let mut result_cache = HashMap::<CacheKey, usize>::new();
    let mut required_unvisited_tiles_map = HashMap::<CacheKey, HashSet<(usize, usize)>>::new();
    let mut work_stack = Vec::<(CacheKey, HashSet<(usize, usize)>)>::new();
    let mut initial_path = HashSet::<(usize, usize)>::new();
    initial_path.insert(*start_pos);
    work_stack.push((start_cache_key.clone(), initial_path));

    while work_stack.len() > 0 {
        let work_item = work_stack.pop().unwrap();

        if work_item.0 == (CacheKey{position: (4,11), incoming_direction: North}) {
            let mut foo = 1;
            foo += 1;
            foo += 1;
        }

        if result_cache.contains_key(&work_item.0) {
            continue;
        }

        if work_item.0 == end_cache_key {
            result_cache.insert(work_item.0.clone(), 1);

            let mut unvisited_tiles_required = HashSet::<(usize, usize)>::new();
            unvisited_tiles_required.insert(work_item.0.position.clone());
            required_unvisited_tiles_map.insert(work_item.0, unvisited_tiles_required);
        } else {
            let mut max_substeps = 0usize;
            let mut max_position_cache_key = None;
            let mut additional_work_items = vec![];

            for next_position in get_valid_moves(&work_item.0.position, map, &work_item.1) {
                let lookup_result = result_cache.get(&next_position);
                if lookup_result.is_some() && cache_constraints_satisfied(&work_item.1, &required_unvisited_tiles_map[&next_position]) {
                    if *lookup_result.unwrap() > max_substeps {
                        max_substeps = *lookup_result.unwrap();
                        max_position_cache_key = Some(next_position);
                    }
                } else {
                    let mut path = work_item.1.clone();
                    path.insert(work_item.0.position);
                    additional_work_items.push((next_position, path));
                }
            }

            // If the child paths have not been found, we need to requeue this spot to ensure
            // this spot's max step count gets updated when all child paths have been found
            if additional_work_items.len() == 0 {
                result_cache.insert(work_item.0.clone(), if max_substeps == 0 { 0 } else { max_substeps + 1 });

                if max_substeps > 0 {
                    let mut unvisited_tiles_required = required_unvisited_tiles_map[&max_position_cache_key.unwrap()].clone();
                    unvisited_tiles_required.insert(work_item.0.position.clone());
                    required_unvisited_tiles_map.insert(work_item.0, unvisited_tiles_required);
                } else {
                    required_unvisited_tiles_map.insert(work_item.0, HashSet::<(usize, usize)>::new());
                }
            } else {
                additional_work_items.push((work_item.0, work_item.1.clone()));
                work_stack.extend(additional_work_items.into_iter().rev());
            }
        }
    }

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            for direction in [North, East, South, West] {
                if let Some(result) = result_cache.get(&CacheKey{ position: (row, col), incoming_direction: direction }) {
                    println!("({}, {}, {:?}) = {}", row, col, direction, result);
                }
            }
        }
    }

    //The initial position doesn't count as a step so subtract 1
    return result_cache[&start_cache_key] - 1;
}

fn cache_constraints_satisfied(path: &HashSet<(usize, usize)>, required_unvisited_tiles: &HashSet<(usize, usize)>) -> bool {
    let result = path.iter().all(|x| !required_unvisited_tiles.contains(x));
    if !result {
        let mut foo = 1;
        foo += 1;
        foo += 1;
    }

    return result;
}
