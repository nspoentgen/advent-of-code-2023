use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_format::{Locale, ToFormattedString};
use Direction::*;

const PATH: char = '.';
const FOREST: char = '#';
const UP_SLOPE: char = '^';
const RIGHT_SLOPE: char = '>';
const DOWN_SLOPE: char = 'v';
const LEFT_SLOPE: char = '<';

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction { North, East, South, West }


fn main() {
    let path = Path::new("src/day23_part1/input.txt");
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

fn get_valid_moves(position: &(usize, usize), map: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) -> Vec<((usize, usize), Direction)> {
    let mut deltas = Vec::<(isize, isize)>::new();
    let position_type = map[position.0][position.1];

    if position_type == UP_SLOPE {
        deltas.push((-1, 0));
    } else if position_type == RIGHT_SLOPE {
        deltas.push((0, 1));
    } else if position_type == DOWN_SLOPE {
        deltas.push((1, 0));
    } else if position_type == LEFT_SLOPE {
        deltas.push((0, -1));
    } else {
        deltas.extend([(0, -1), (0, 1), (-1, 0), (1, 0)]);
    }

    let signed_position = (position.0 as isize, position.1 as isize);
    let mut valid_positions = Vec::<((usize, usize), Direction)>::new();

    for delta in deltas {
        let test_position = (signed_position.0 + delta.0, signed_position.1 + delta.1);
        if test_position.0 >= 0 && test_position.0 < map.len() as isize && test_position.1 >= 0 &&
            test_position.1 < map[0].len() as isize && map[test_position.0 as usize][test_position.1 as usize] != FOREST &&
            !path.contains(&(test_position.0 as usize, test_position.1 as usize))
        {
            let unsigned_test_position = (test_position.0 as usize, test_position.1 as usize);
            let direction = get_direction(&position, &unsigned_test_position);
            valid_positions.push((unsigned_test_position, direction));
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
    const START_DIRECTION: Direction = South;
    const END_DIRECTION: Direction = South;

    let mut cache = HashMap::<((usize, usize), Direction), usize>::new();
    let mut work_stack = Vec::<(((usize, usize), Direction), HashSet<(usize, usize)>)>::new();
    let mut initial_path = HashSet::<(usize, usize)>::new();
    initial_path.insert(*start_pos);
    work_stack.push(((*start_pos, START_DIRECTION), initial_path));

    let mut counter = 0u64;

    while work_stack.len() > 0 {
        let work_item = work_stack.pop().unwrap();

        counter += 1;
        //println!("Iteration {}, position = ({}, {}), direction = {:?}", counter, work_item.0.0.0, work_item.0.0.1, work_item.0.1);

        if cache.contains_key(&(work_item.0.0, work_item.0.1)) {
            continue;
        }

        if work_item.0 == (*end_pos, END_DIRECTION) {
            cache.insert(work_item.0, 1);
        } else {
            let mut max_substeps = 0usize;
            let mut additional_work_items = vec![];

            for next_position in get_valid_moves(&work_item.0.0, map, &work_item.1) {
                if let Some(result) = cache.get(&next_position) {
                    if *result > max_substeps {
                        max_substeps = *result;
                    }
                } else {
                    let mut path = work_item.1.clone();
                    path.insert(work_item.0.0);
                    additional_work_items.push((next_position, path));
                }
            }

            // If the child paths have not been found, we need to requeue this spot to ensure
            // this spot's max step count gets updated when all child paths have been found
            if additional_work_items.len() == 0 {
                cache.insert(work_item.0, if max_substeps == 0 { 0 } else { max_substeps + 1 });
            } else {
                additional_work_items.push((work_item.0, work_item.1.clone()));
                work_stack.extend(additional_work_items.into_iter().rev());
            }
        }
    }

    /*
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            for direction in [North, East, South, West] {
                if let Some(result) = cache.get(&((row, col), direction)) {
                    println!("({}, {}) = {}", row, col, result);
                }
            }
        }
    }
     */

    //The initial position doesn't count as a step so subtract 1
    return cache[&(*start_pos, END_DIRECTION)] - 1;
}