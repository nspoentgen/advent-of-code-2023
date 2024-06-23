mod brick;

use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use brick::Orientation;
use crate::brick::Brick;

type Coordinate = (u64, u64, u64);

fn main() {
    //Parse data and init world map
    let path = Path::new("src/day22_part1/input.txt");
    let mut bricks = parse_data(&path);
    let mut world_map = HashMap::<Coordinate, u32>::new();
    initialize_world_map(&bricks, &mut world_map);

    //Let bricks fall and then disintegrate
    get_final_positions(&mut bricks, &mut world_map);
    let num_bricks_can_be_removed = bricks
        .iter()
        .map(|x| brick_can_be_disintegrated(x, &bricks, &mut world_map))
        .filter(|x| *x)
        .count();

    println!("The number of bricks that can be removed is {}", num_bricks_can_be_removed);
}

fn parse_data(path: &Path) -> Vec<Brick> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .into_iter()
        .map(parse_positions)
        .enumerate()
        .map(|(index, positions)| Brick::new(index as u32, positions[0], positions[1]))
        .collect_vec();
}

fn parse_positions(line: String) -> Vec<Coordinate> {
    return line
        .split("~")
        .map(|pos| pos.split(",").map(|term| term.parse::<u64>().unwrap()).collect_vec())
        .map(|x| (x[0], x[1], x[2]))
        .collect_vec();
}

fn initialize_world_map(bricks: &Vec<Brick>, world_map: &mut HashMap<Coordinate, u32>) {
    for brick in bricks {
        insert_brick(&brick, world_map);
    }
}

fn insert_brick(brick: &Brick, world_map: &mut HashMap<Coordinate, u32>) {
    match brick.orientation {
        Orientation::X => {
            for x in brick.start_position.0..=brick.end_position.0 {
                world_map.insert((x, brick.start_position.1, brick.start_position.2), brick.id);
            }
        },
        Orientation::Y => {
            for y in brick.start_position.1..=brick.end_position.1 {
                world_map.insert((brick.start_position.0, y, brick.start_position.2), brick.id);
            }
        },
        Orientation::Z => {
            for z in brick.start_position.2..=brick.end_position.2 {
                world_map.insert((brick.start_position.0, brick.start_position.1, z), brick.id);
            }
        }
    };
}

fn remove_brick(brick: &Brick, world_map: &mut HashMap<Coordinate, u32>) {
    match brick.orientation {
        Orientation::X => {
            for x in brick.start_position.0..=brick.end_position.0 {
                world_map.remove(&(x, brick.start_position.1, brick.start_position.2));
            }
        },
        Orientation::Y => {
            for y in brick.start_position.1..=brick.end_position.1 {
                world_map.remove(&(brick.start_position.0, y, brick.start_position.2));
            }
        },
        Orientation::Z => {
            for z in brick.start_position.2..=brick.end_position.2 {
                world_map.remove(&(brick.start_position.0, brick.start_position.1, z));
            }
        }
    };
}

fn get_final_positions(bricks: &mut Vec<Brick>, world_map: &mut HashMap<Coordinate, u32>) {
    let mut brick_moved = true;

    while brick_moved {
        brick_moved = false;

        for brick in &mut *bricks {
            if brick.can_fall_one(world_map) {
                brick.fall(1, world_map);
                brick_moved = true;
            }
        }
    }
}

fn brick_can_be_disintegrated(test_brick: &Brick, bricks: &Vec<Brick>, world_map: &mut HashMap<Coordinate, u32>) -> bool {
    let mut can_be_removed = true;
    remove_brick(&test_brick, world_map);

    for other_brick in bricks.iter().filter(|&x| *x != *test_brick) {
        if other_brick.can_fall_one(world_map) {
            can_be_removed = false;
            break;
        }
    }

    insert_brick(test_brick, world_map);
    return can_be_removed;
}
