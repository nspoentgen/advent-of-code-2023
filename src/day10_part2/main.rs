use std::collections::{HashSet, HashMap};
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
    solver.calc_path_normals();

    //Put data in hash tables for next step
    let mut path_set = HashSet::<Point>::new();
    for path_point in solver.get_path() {
        path_set.insert(path_point.clone());
    }
    let mut path_normals_map = HashMap::<Point, NormalDirection>::new();
    for index in 0..solver.get_path().len() {
        path_normals_map.insert(solver.get_path()[index].clone(), solver.get_path_normals()[index].clone());
    }

    //Calculate and print final solution
    let num_encircled_times = calc_num_encircled_tiles(&path_set, &path_normals_map, grid_map.len(), grid_map[0].len());
    println!("The number of encircled tiles is {}", num_encircled_times.to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> GridMatrix {
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
        .collect::<GridMatrix>();
}

fn find_starting_position(grid_map: &GridMatrix) -> Point {
    for row in 0..grid_map.len() {
        for col in 0..grid_map[row].len() {
            if grid_map[row][col] == GridItem::StartingPosition {
                return (row, col);
            }
        }
    }

    panic!("Could not find starting position");
}

fn calculate_starting_grid_item(starting_position: &Point, grid_map: &GridMatrix) -> GridItem {
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

fn calc_num_encircled_tiles(path_set: &HashSet<Point>, path_normals_map: &HashMap<Point, NormalDirection>, grid_row_max: usize, grid_col_max: usize) -> u32 {
    let mut num_encircled_tiles = 0u32;

    for row_index in 0..grid_row_max {
        for col_index in 0..grid_col_max {
            let test_point = &(row_index, col_index);

            if !path_set.contains(test_point) &&
               ray_bounded_in_path(test_point, North, &path_set, &path_normals_map, grid_row_max, grid_col_max) &&
               ray_bounded_in_path(test_point, East, &path_set, &path_normals_map, grid_row_max, grid_col_max) &&
               ray_bounded_in_path(test_point, South, &path_set, &path_normals_map, grid_row_max, grid_col_max) &&
               ray_bounded_in_path(test_point, West, &path_set, &path_normals_map, grid_row_max, grid_col_max) {
                num_encircled_tiles += 1;
            }
        }
    }

    return num_encircled_tiles;
}

fn ray_bounded_in_path(ray_origin: &Point, direction: RelativePosition, path_set: &HashSet<Point>, path_normals_map: &HashMap<Point, NormalDirection>, grid_row_max: usize, grid_col_max: usize) -> bool {
    let point_iter: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
        North => Box::new((0..ray_origin.0 + 1).rev().map(|row_index| (row_index, ray_origin.1))),
        East => Box::new((ray_origin.1..grid_col_max + 1).map(|col_index| (ray_origin.0, col_index))),
        South => Box::new((ray_origin.0..grid_row_max + 1).map(|row_index| (row_index, ray_origin.1))),
        West => Box::new((0..ray_origin.1 + 1).rev().map(|col_index| (ray_origin.0, col_index))),
    };

    let inverse_normal: Vec<NormalDirection> = match direction {
        North => vec![NormalDirection::South, NormalDirection::Southwest, NormalDirection::Southeast],
        East => vec![NormalDirection::West, NormalDirection::Northwest, NormalDirection::Southwest],
        South => vec![NormalDirection::North, NormalDirection::Northeast, NormalDirection::Northwest],
        West => vec![NormalDirection::East, NormalDirection::Northeast, NormalDirection::Southeast]
    };

    let mut contained = false;
    for point in point_iter {
        if path_set.contains(&point) {
            contained = inverse_normal.contains(&path_normals_map[&point]);
            break;
        }
    }

    return contained;
}
