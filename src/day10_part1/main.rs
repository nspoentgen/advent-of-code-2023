use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

use crate::grid_item::*;
use crate::grid_item::GridItem::{Horizontal, RightAngleNorthEast, RightAngleNorthWest, RightAngleSouthEast, RightAngleSouthWest, Vertical};
use crate::grid_item::RelativePosition::*;
use crate::pipe_loop_solver::*;

mod grid_item;
mod pipe_loop_solver;

fn main() {
    //Parse input data, find starting position, and update staring position with calculated grid item
    let path = Path::new("src/day10_part1/input.txt");
    let mut grid_map = parse_data(path);
    let starting_position = find_starting_position(&grid_map);
    let starting_grid_item = calculate_starting_grid_item(&starting_position, &grid_map);
    grid_map[starting_position.0][starting_position.1] = starting_grid_item;

    //Solve grid
    let mut solver = Solver::new(&starting_position, &grid_map);
    solver.solve();

    //Print result
    println!("The maximum number of steps in the loop is {}", (solver.get_path().len() / 2).to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<Vec<GridItem>> {
    let parse_line = |line: &String| -> Vec<GridItem> {
        return line
            .chars()
            .map(|x| GridItem::parse(x))
            .collect::<Vec<GridItem>>();
    };

    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_line(&l))
        .collect::<Vec<Vec<GridItem>>>();
}

fn find_starting_position(grid_map: &Vec<Vec<GridItem>>) -> (usize, usize) {
    for row in 0..grid_map.len() {
        for col in 0..grid_map[row].len() {
            if grid_map[row][col] == GridItem::StartingPosition {
                return (row, col);
            }
        }
    }

    panic!("Could not find starting position");
}

fn calculate_starting_grid_item(starting_position: &(usize, usize), grid_map: &Vec<Vec<GridItem>>) -> GridItem {
    let starting_options_relative: (RelativePosition, RelativePosition) = get_valid_positions(starting_position, starting_position, grid_map)
        .iter()
        .map(|test_pos| get_relative_position(starting_position, test_pos))
        .collect_tuple()
        .unwrap();

    return if starting_options_relative == (North, South) || starting_options_relative == (South, North) {
        Vertical
    } else if starting_options_relative == (North, East) || starting_options_relative == (East, North) {
        RightAngleNorthEast
    } else if starting_options_relative == (North, West) || starting_options_relative == (West, North) {
        RightAngleNorthWest
    } else if starting_options_relative == (East, South) || starting_options_relative == (South, East) {
      RightAngleSouthEast
    } else if starting_options_relative == (East, West) || starting_options_relative == (West, East) {
        Horizontal
    } else if starting_options_relative == (South, West) || starting_options_relative == (West, South) {
        RightAngleSouthWest
    } else {
        panic!("Could not determine starting grid item type")
    };
}
