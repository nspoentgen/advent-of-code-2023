use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};
use num_format::Locale::{am, br, se};

type Coordinate = (u64, u64, u64);

#[derive(Eq, PartialEq, Hash)]
enum Orientation {
    X, Y, Z
}

#[derive(Eq, PartialEq, Hash)]
struct Brick {
    pub id: u32,
    pub start_position: Coordinate,
    pub end_position: Coordinate,
    pub orientation: Orientation
}

impl Brick {
    fn new(id: u32, start_position: Coordinate, end_position: Coordinate) -> Self {
        let orientation = if start_position.0 == end_position.0 {
            Orientation::X
        } else if start_position.1 == end_position.1 {
            Orientation::Y
        } else {
            Orientation::Z
        };

        return Brick { id, start_position, end_position, orientation};
    }

    fn can_fall_one(&self, world_map: &mut HashMap<Coordinate, u32>) -> bool {
        if self.start_position.2 == 1 || self.end_position.2 == 1 {
            return false;
        }

        let test_start_position = (self.start_position.0, self.start_position.1, self.start_position.2 - 1);
        let start_result = world_map.get_key_value(&test_start_position);

        let test_end_position = (self.end_position.0, self.end_position.1, self.end_position.2 - 1);
        let end_result = world_map.get_key_value(&test_end_position);

        return (start_result.is_none() || *start_result.unwrap().1 == self.id) &&
            (end_result.is_none() || *end_result.unwrap().1 == self.id);
    }

    fn supports_a_brick(&self, world_map: &HashMap<Coordinate, u32>) -> bool {
        let test_start_position = (self.start_position.0, self.start_position.1, self.start_position.2 + 1);
        let start_result = world_map.get_key_value(&test_start_position);

        let test_end_position = (self.end_position.0, self.end_position.1, self.end_position.2 + 1);
        let end_result = world_map.get_key_value(&test_end_position);

        return start_result.is_some_and(|x| *x.1 != self.id) &&
            end_result.is_some_and(|x| *x.1 != self.id);
    }

    fn fall(&mut self, amount: u64, world_map: &mut HashMap<Coordinate, u32>) {
        let old_start_position = self.start_position;
        let old_end_position = self.end_position;
        self.start_position.2 -= amount;
        self.end_position.2 -= amount;

        match self.orientation {
            Orientation::X => {
                for x in old_start_position.0..=old_end_position.0 {
                    world_map.remove(&(x, old_start_position.1, old_start_position.2));
                    world_map.insert((x, self.start_position.1, self.start_position.2), self.id);
                }
            },
            Orientation::Y => {
                for y in old_start_position.1..=old_end_position.1 {
                    world_map.remove(&(old_start_position.0, y, old_start_position.2));
                    world_map.insert((self.start_position.0, y, self.start_position.2), self.id);
                }
            },
            Orientation::Z => {
                for z in old_start_position.2..=old_end_position.2 {
                    world_map.remove(&(old_start_position.0, old_start_position.1, z));
                    world_map.insert((self.start_position.0, self.start_position.1, z - amount), self.id);
                }
            }
        };
    }
}


fn main() {
    //Parse data and init world map
    let path = Path::new("src/day22_part1/test_input.txt");
    let mut bricks = parse_data(&path);
    let mut world_map = HashMap::<Coordinate, u32>::new();
    initialize_world_map(&bricks, &mut world_map);

    //Let bricks fall and then disintegrate
    get_final_positions(&mut bricks, &mut world_map);
    let mut disintegrated_bricks = HashSet::<Brick>::from_iter(bricks.into_iter());
    disintegrate_bricks(&mut disintegrated_bricks, &mut world_map);

    println!("The number of bricks is {}", disintegrated_bricks.len());
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

fn disintegrate_bricks(bricks: &mut HashSet<Brick>, world_map: &mut HashMap<Coordinate, u32>) {
    let mut brick_disintegrated = true;
    let mut bricks_to_remove = Vec::<&Brick>::new();

    while brick_disintegrated {
        brick_disintegrated = false;

        for brick in bricks.iter() {
            if brick.supports_a_brick(world_map) {
                bricks_to_remove.push(brick);
                brick_disintegrated = true;
            }
        }

        for brick in bricks_to_remove {
            bricks.remove(brick);
        }
        bricks_to_remove.clear();
    }
}
