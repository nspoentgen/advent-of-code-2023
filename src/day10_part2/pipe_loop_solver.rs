use std::collections::{HashSet};
use crate::grid_item::*;
use crate::grid_item::GridItem::*;

//Types
pub type Point = (usize, usize);
pub type GridMatrix = Vec<Vec<GridItem>>;

//Structs
pub struct Solver<'a> {
    starting_position: Point,
    grid_map: &'a GridMatrix,
    path: Vec<Point>,
    path_normals: Vec<NormalDirection>
}

impl<'a> Solver<'a> {
    pub fn get_path(&self) -> &Vec<Point> {
        return &self.path;
    }

    pub fn get_path_normals(&self) -> &Vec<NormalDirection> {
        return &self.path_normals;
    }

    pub fn new(starting_position: &Point, grid_map: &'a GridMatrix) -> Self {
        return Self {
            starting_position: starting_position.clone(),
            grid_map,
            path: Vec::<Point>::new(),
            path_normals: Vec::<NormalDirection>::new()
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

    pub fn calc_path_normals(&mut self) {
        //First find an outside point. We will use any point on the top most edge.
        let mut topmost_point = &self.get_path()[0];
        let mut topmost_index = 0usize;

        for index in 1..self.path.len() {
            if self.path[index].0 < topmost_point.0 {
                topmost_index = index;
                topmost_point = &self.path[topmost_index];
            }
        }

        //Determine the initial point's normal direction.
        let initial_normal = match self.grid_map[topmost_point.0][topmost_point.1] {
            Horizontal => NormalDirection::South,
            RightAngleSouthEast => NormalDirection::Southeast,
            RightAngleSouthWest => NormalDirection::Southwest,
            _ => panic!("Bad initial mapping for path normals calculation")
        };

        //Initialize our path normals list with unknowns except for the starting point
        self.path_normals.clear();
        for index in 0..self.path.len() {
            self.path_normals.push(if index == topmost_index { initial_normal } else { NormalDirection::Unknown});
        }

        //Use induction to calculate the rest of the normals
        for index in topmost_index..self.path_normals.len() {
            let next_index = (index + 1) % self.path_normals.len();
            self.path_normals[next_index] = self.get_next_normal(index, next_index);
        }

        for index in 0..=topmost_index - 2 {
            let next_index = (index + 1) % self.path_normals.len();
            self.path_normals[next_index] = self.get_next_normal(index, next_index);
        }
    }

    fn get_next_normal(&self, index: usize, next_index: usize) -> NormalDirection {


        let current_position = &self.path[index];
        let next_position = &self.path[next_index];
        //println!("{:?}", (&self.grid_map[current_position.0][current_position.1], &self.grid_map[next_position.0][next_position.1], &self.path_normals[index]));
        return *NORMAL_LOOKUP_TABLE[&(&self.grid_map[current_position.0][current_position.1],
                                      &self.grid_map[next_position.0][next_position.1], &self.path_normals[index])];
    }
}

//Functions
pub fn get_valid_positions(position: &Point, prev_position: &Point, grid_map: &GridMatrix) -> Vec<Point> {
    let mut test_positions = HashSet::from([
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1)
    ]);
    test_positions.remove(&prev_position);

    let is_inbounds = |test_pos: &Point| test_pos.0 < grid_map.len() &&
        test_pos.1 < grid_map[0].len();

    return test_positions
        .iter()
        .filter(|&test_pos| is_inbounds(test_pos))
        .filter(|&test_pos| is_valid_join(position, test_pos, grid_map))
        .map(|x| x.clone())
        .collect::<Vec<Point>>()
}

pub fn is_valid_join(position: &Point, test_pos: &Point, grid_map: &GridMatrix) -> bool {


    let current_grid_item = &grid_map[position.0][position.1];
    let test_grid_item = &grid_map[test_pos.0][test_pos.1];
    let relative_position = get_relative_position(position, test_pos);
    let is_valid = VALID_JOIN_COMINATIONS.contains(&(current_grid_item, test_grid_item, &relative_position));

    return is_valid;
}
