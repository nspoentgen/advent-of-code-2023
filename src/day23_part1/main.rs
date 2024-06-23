use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

const PATH: char = '.';
const FOREST: char = '#';
const UP_SLOPE: char = '^';
const RIGHT_SLOPE: char = '>';
const DOWN_SLOPE: char = 'v';
const LEFT_SLOPE: char = '<';


fn main() {
    let path = Path::new("src/day23_part1/test_input.txt");
    let map = parse_data(&path);
    let (start_pos, end_pos) = find_terminal_positions(&map);
    let max_steps = get_max_steps(&start_pos, &end_pos, &map);

    println!("The maximum number of steps is {}", max_steps);
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

fn get_valid_moves(position: &(usize, usize), map: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    let deltas: [(isize, isize); 4] = [(0,-1), (0,1), (-1,0), (1,0)];

    let signed_position = (position.0 as isize, position.1 as isize);
    let mut valid_positions = Vec::<(usize, usize)>::new();

    for delta in deltas {
        let test_position = (signed_position.0 + delta.0, signed_position.1 + delta.1);
        if test_position.0 >= 0 && test_position.0 < map.len() as isize && test_position.1 >= 0 &&
            test_position.1 < map[0].len() as isize && map[test_position.0 as usize][test_position.1 as usize] != FOREST &&
            !path.contains(&(test_position.0 as usize, test_position.1 as usize))
        {
            valid_positions.push((test_position.0 as usize, test_position.1 as usize));
        }
    }

    return valid_positions;
}

fn get_max_steps(start_pos: &(usize, usize), end_pos: &(usize, usize), map: &Vec<Vec<char>>) -> usize {
    let mut max_steps = 0usize;
    let mut cache = HashMap::<(usize, usize), usize>::new();
    let mut work_stack = VecDeque::<((usize, usize), HashSet<(usize, usize)>)>::new();
    let mut initial_path = HashSet::<(usize, usize)>::new();
    initial_path.insert(*start_pos);
    work_stack.push_front((*start_pos, initial_path));

    while work_stack.len() > 0 {
        let mut work_item = work_stack.pop_front().unwrap();
        let next_positions = get_valid_moves(&work_item.0, map, &work_item.1);

        for position in next_positions {
            if let Some(future_steps) = cache.get(&(position)) {
                let num_steps = work_item.1.len() + 1 + *future_steps;
                cache.insert(position, num_steps);

                if num_steps > max_steps {
                    max_steps = num_steps;
                }
            }
            else if position == *end_pos {
                cache.insert(position, work_item.1.len() + 1);
            } else {
                let mut path = work_item.1.clone();
                path.insert(work_item.0);
                work_stack.push_front((position, path));
            }
        }
    }

    return if max_steps == 0 { 0 } else { max_steps - 1 };
}