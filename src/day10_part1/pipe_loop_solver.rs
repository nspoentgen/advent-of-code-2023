use std::collections::HashSet;
use crate::grid_item::*;
use crate::grid_item::GridItem::*;
use crate::grid_item::RelativePosition::*;

pub struct Solver<'a> {
    starting_position: (usize, usize),
    grid_map: &'a Vec<Vec<GridItem>>,
    path: Vec<(usize, usize)>
}

impl<'a> Solver<'a> {
    pub fn get_path(&self) -> &Vec<(usize, usize)> {
        return &self.path;
    }

    pub fn new(starting_position: &(usize, usize), grid_map: &'a Vec<Vec<GridItem>>) -> Self {
        return Self {
            starting_position: starting_position.clone(),
            grid_map,
            path: Vec::<(usize, usize)>::new(),
        };
    }

    pub fn solve(&mut self) {
        self.path.push(self.starting_position);
        let mut loop_complete = false;

        while !loop_complete {
            let current_position = self.path.last().unwrap();
            let prev_position = self.path.iter().nth(self.path.len().wrapping_sub(2)).unwrap_or_else(|| current_position);
            let valid_positions = get_valid_positions(&current_position, &prev_position, self.grid_map);
            let next_point = valid_positions.first().unwrap().clone();

            loop_complete = next_point == self.starting_position;
            if !loop_complete {
                self.path.push(next_point);
            }
        }
    }
}

pub fn get_valid_positions(position: &(usize, usize), prev_position: &(usize, usize), grid_map: &Vec<Vec<GridItem>>) -> Vec<(usize, usize)> {
    let mut test_positions = HashSet::from([
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1)
    ]);
    test_positions.remove(&prev_position);

    let is_inbounds = |test_pos: &(usize, usize)| test_pos.0 < grid_map.len() &&
        test_pos.1 < grid_map[0].len();

    return test_positions
        .iter()
        .filter(|&test_pos| is_inbounds(test_pos))
        .filter(|&test_pos| is_valid_join(position, test_pos, grid_map))
        .map(|x| x.clone())
        .collect::<Vec<(usize, usize)>>()
}

pub fn is_valid_join(position: &(usize, usize), test_pos: &(usize, usize), grid_map: &Vec<Vec<GridItem>>) -> bool {
    let valid_combinations: HashSet<(&GridItem, &GridItem, &RelativePosition)> = HashSet::from(
        [(&Vertical, &Vertical, &North),
            (&Vertical, &Vertical, &South),
            (&Vertical, &RightAngleNorthEast, &South),
            (&Vertical, &RightAngleNorthWest, &South),
            (&Vertical, &RightAngleSouthWest, &North),
            (&Vertical, &RightAngleSouthEast, &North),
            (&Horizontal, &Horizontal, &West),
            (&Horizontal, &Horizontal, &East),
            (&Horizontal, &RightAngleNorthEast, &West),
            (&Horizontal, &RightAngleNorthWest, &East),
            (&Horizontal, &RightAngleSouthWest, &East),
            (&Horizontal, &RightAngleSouthEast, &West),
            (&RightAngleNorthEast, &Vertical, &North),
            (&RightAngleNorthEast, &Horizontal, &East),
            (&RightAngleNorthEast, &RightAngleNorthWest, &East),
            (&RightAngleNorthEast, &RightAngleSouthWest, &North),
            (&RightAngleNorthEast, &RightAngleSouthWest, &East),
            (&RightAngleNorthEast, &RightAngleSouthEast, &North),
            (&RightAngleNorthWest, &Vertical, &North),
            (&RightAngleNorthWest, &Horizontal, &West),
            (&RightAngleNorthWest, &RightAngleNorthEast, &West),
            (&RightAngleNorthWest, &RightAngleSouthWest, &North),
            (&RightAngleNorthWest, &RightAngleSouthEast, &North),
            (&RightAngleNorthWest, &RightAngleSouthEast, &West),
            (&RightAngleSouthWest, &Vertical, &South),
            (&RightAngleSouthWest, &Horizontal, &West),
            (&RightAngleSouthWest, &RightAngleNorthEast, &West),
            (&RightAngleSouthWest, &RightAngleNorthEast, &South),
            (&RightAngleSouthWest, &RightAngleNorthWest, &South),
            (&RightAngleSouthWest, &RightAngleSouthEast, &West),
            (&RightAngleSouthEast, &Vertical, &South),
            (&RightAngleSouthEast, &Horizontal, &East),
            (&RightAngleSouthEast, &RightAngleNorthEast, &South),
            (&RightAngleSouthEast, &RightAngleNorthWest, &South),
            (&RightAngleSouthEast, &RightAngleNorthWest, &East),
            (&RightAngleSouthEast, &RightAngleSouthWest, &East),
            (&StartingPosition, &Vertical, &North),
            (&StartingPosition, &Vertical, &South),
            (&StartingPosition, &Horizontal, &West),
            (&StartingPosition, &Horizontal, &East),
            (&StartingPosition, &RightAngleNorthEast, &South),
            (&StartingPosition, &RightAngleNorthEast, &West),
            (&StartingPosition, &RightAngleNorthWest, &South),
            (&StartingPosition, &RightAngleNorthWest, &East),
            (&StartingPosition, &RightAngleSouthWest, &North),
            (&StartingPosition, &RightAngleSouthWest, &East),
            (&StartingPosition, &RightAngleSouthEast, &North),
            (&StartingPosition, &RightAngleSouthEast, &West)
        ]);

    let current_grid_item = &grid_map[position.0][position.1];
    let test_grid_item = &grid_map[test_pos.0][test_pos.1];
    let relative_position = get_relative_position(position, test_pos);
    let is_valid = valid_combinations.contains(&(current_grid_item, test_grid_item, &relative_position));

    return is_valid;
}
