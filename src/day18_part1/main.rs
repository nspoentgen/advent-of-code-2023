mod grid_data;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use itertools::Itertools;
use grid_data::*;
use grid_data::PerimeterMovement::*;
use grid_data::Orientation::*;
use num_format::{Locale, ToFormattedString};

type DigInfo = (char, isize, String);
const CENTER_TO_BOTTOM_LEFT_CORNER_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (-SQUARE_EDGE_LENGTH / 2f32, -SQUARE_EDGE_LENGTH/2f32)};
const CENTER_TO_UPPER_LEFT_CORNER_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (-SQUARE_EDGE_LENGTH / 2f32, SQUARE_EDGE_LENGTH/2f32)};
const NORTH_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (0f32, SQUARE_EDGE_LENGTH)};
const SOUTH_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (0f32, -SQUARE_EDGE_LENGTH)};
const WEST_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (-SQUARE_EDGE_LENGTH, 0f32)};
const EAST_OFFSET: DiscreteCoordinateFloat = DiscreteCoordinateFloat {point: (SQUARE_EDGE_LENGTH, 0f32)};

fn main () {
    //Parse data
    let path = Path::new("src/day18_part1/input.txt");
    let dig_data = parse_data(path);

    //Get polygon boundary as list of ordered vertices in CCW order
    let mut polygon_ordered_vertices = generate_polygon_edge_segment_vertices(&dig_data);
    let lower_left_corner_index = find_lower_left_corner(&polygon_ordered_vertices);
    enforce_counter_clockwise_boundary(&mut polygon_ordered_vertices, lower_left_corner_index);

    //Calculate and print area. Note that the edges are subdivided into line segments, so these are
    //polygon edge segments, not edges. However, for the purpose of the shoelace theorem algorithm, this
    //does not matter; the calculation is just less efficient.
    let area = calculate_area_shoelace_theorem(&polygon_ordered_vertices);
    println!("Area = {}", (area.round() as i32).to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> Vec<DigInfo> {
    let file = File::open(&path).unwrap();
    return BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            (parts[0].chars().nth(0).unwrap(), parts[1].parse::<isize>().unwrap(), parts[2].to_string())
        })
        .collect();
}

//Generates vertices that define the outer perimeter of the polygon. However, the perimeter is composed of many
//line segments. A future method combines these line segments into a proper polygon edges.
fn generate_polygon_edge_segment_vertices(dig_data: &Vec<DigInfo>) -> Vec<DiscreteCoordinateInt> {
    //Initialization logic
    let square_centers = generate_square_centers(dig_data);
    let start_index = find_leftmost_center_point(&square_centers);
    let (mut edge_segment_vertices, orientation) = initialize_edge_segment_vertices(&square_centers, start_index);

    //Propagate forward using inductive logic
    for index_offset in 1..square_centers.len() {
        add_next_perimeter_vertex(&mut edge_segment_vertices, start_index + index_offset, orientation, &square_centers);
    }

    //We know vertices are on integer boundaries so cast to integer coordinates
    return edge_segment_vertices.iter().map(|x| x.round_to_discrete_coordinate_int()).collect_vec();
}

fn generate_square_centers(dig_data: &Vec<DigInfo>) -> Vec<DiscreteCoordinateFloat> {
    //Declare starting square as origin
    let mut square_centers = vec![(0.5f32, 0.5f32)];
    let mut prev_center = square_centers[0];

    for info in dig_data {
        for _ in 0..info.1 {
            let next_center = match info.0 {
                'U' => (prev_center.0, prev_center.1 + SQUARE_EDGE_LENGTH),
                'R' => (prev_center.0 + SQUARE_EDGE_LENGTH, prev_center.1),
                'D' => (prev_center.0, prev_center.1 - SQUARE_EDGE_LENGTH),
                'L' => (prev_center.0 - SQUARE_EDGE_LENGTH, prev_center.1),
                _ => panic!("Direction not mapped")
            };
            square_centers.push(next_center.clone());
            prev_center = next_center;
        }
    }

    //First and last elements overlap to form a loop. Remove duplicate
    square_centers.pop();
    return square_centers.iter().map(|center| DiscreteCoordinateFloat {point: *center}).collect_vec();
}

//Initialize the perimeter segment vertices on a left-most edge. Corners are special cases where
//we need to add two points.
fn initialize_edge_segment_vertices(square_centers: &Vec<DiscreteCoordinateFloat>, start_index: usize) -> (Vec<DiscreteCoordinateFloat>, Orientation) {
    let mut edge_segment_vertices = Vec::<DiscreteCoordinateFloat>::new();
    let index_n_minus_one = (start_index + square_centers.len() - 1) % square_centers.len();
    let index_n = start_index;
    let index_n_plus_one = (start_index + 1) % square_centers.len();
    let orientation: Orientation;

    match get_perimeter_movement(&square_centers[index_n_minus_one], &square_centers[index_n], &square_centers[index_n_plus_one]) {
        North => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_UPPER_LEFT_CORNER_OFFSET);
            orientation = CW;
        },
        NorthToEast => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_UPPER_LEFT_CORNER_OFFSET);
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
            orientation = CW;
        },
        WestToNorth => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_BOTTOM_LEFT_CORNER_OFFSET);
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
            orientation = CW;
        },
        South => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_BOTTOM_LEFT_CORNER_OFFSET);
            orientation = CCW;
        },
        SouthToEast => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_BOTTOM_LEFT_CORNER_OFFSET);
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
            orientation = CCW;
        },
        WestToSouth => {
            edge_segment_vertices.push(&square_centers[start_index] + CENTER_TO_UPPER_LEFT_CORNER_OFFSET);
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
            orientation = CCW;
        },
        _ => panic!("Logic error. Starting condition should not be possible")
    }
    
    return (edge_segment_vertices, orientation);
}

fn add_next_perimeter_vertex(edge_segment_vertices: &mut Vec<DiscreteCoordinateFloat>, current_index: usize, orientation: Orientation,
                             square_centers: &Vec<DiscreteCoordinateFloat>) {
    let index_n_minus_one = (current_index + square_centers.len() - 1) % square_centers.len();
    let index_n = (current_index) % square_centers.len();
    let index_n_plus_one = (current_index + 1) % square_centers.len();

    match get_perimeter_movement(&square_centers[index_n_minus_one], &square_centers[index_n], &square_centers[index_n_plus_one]) {
        North => {
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
        },
        East => {
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
        },
        South => {
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
        },
        West => {
            edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + WEST_OFFSET);
        },
        NorthToEast => {
            if orientation == CW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
            }
        },
        EastToSouth => {
            if orientation == CW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
            }
        },
        SouthToWest => {
            if orientation == CW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + WEST_OFFSET);
            }
        },
        WestToNorth => {
            if orientation == CW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + WEST_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
            }
        },
        WestToSouth => {
            if orientation == CCW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + WEST_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
            }
        },
        SouthToEast => {
            if orientation == CCW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + SOUTH_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
            }
        },
        EastToNorth => {
            if orientation == CCW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + EAST_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
            }
        },
        NorthToWest => {
            if orientation == CCW {
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + NORTH_OFFSET);
                edge_segment_vertices.push(&edge_segment_vertices[edge_segment_vertices.len() - 1] + WEST_OFFSET);
            }
        }
    }
}

//If more than one, choose first
fn find_leftmost_center_point(centers: &Vec<DiscreteCoordinateFloat>) -> usize {
    let mut leftmost_x_coordinate: Option<f32> = None;
    let mut leftmost_index = 0usize;

    for (test_index, test_x_coordinate) in centers.iter().map(|c| c.point.0).enumerate() {
        if leftmost_x_coordinate.is_none() || test_x_coordinate < leftmost_x_coordinate.unwrap() {
            leftmost_x_coordinate = Some(test_x_coordinate);
            leftmost_index = test_index;
        }
    }

    return leftmost_index;
}

fn find_lower_left_corner(vertices: &Vec<DiscreteCoordinateInt>) -> usize {
    let mut lower_left_vertex = (i32::MAX, i32::MAX);
    let mut lower_left_index = usize::MAX;

    for index in 0..vertices.len() {
        if vertices[index].0 < lower_left_vertex.0 {
            lower_left_vertex = vertices[index];
            lower_left_index = index;
        } else if vertices[index].0 == lower_left_vertex.0 && vertices[index].1 < lower_left_vertex.1 {
            lower_left_vertex = vertices[index];
            lower_left_index = index;
        }
    }

    return lower_left_index
}

fn enforce_counter_clockwise_boundary(boundary: &mut Vec<DiscreteCoordinateInt>, lower_left_corner_index: usize) {
    //Get the previous and next index to form an angle
    let first_test_index = lower_left_corner_index;
    let zeroth_test_index = (first_test_index + boundary.len() - 1) % boundary.len();
    let second_test_index = (first_test_index + 1) % boundary.len();

    //Compute the cross product of the orientation matrix to determine the orientation of our
    //boundary
    let xa = boundary[zeroth_test_index].0 as f32;
    let ya = boundary[zeroth_test_index].1 as f32;
    let xb = boundary[first_test_index].0 as f32;
    let yb = boundary[first_test_index].1 as f32;
    let xc = boundary[second_test_index].0 as f32;
    let yc = boundary[second_test_index].1 as f32;
    let vector_cross_product = (xb*yc + xa*yb + ya*xc) - (ya*xb + yb*xc + xa*yc);

    //If orientation is not already CCW, then reverse the order to make it CCW.
    let counter_clockwise = vector_cross_product >= 0f32;
    if !counter_clockwise {
        boundary.reverse();
    }
}

//Must be in CCW order
fn calculate_area_shoelace_theorem(vertices: &Vec<DiscreteCoordinateInt>) -> f32 {
    let mut terms = Vec::<f32>::new();
    for index in 0..vertices.len() {
        terms.push(((vertices[index].1 + vertices[(index + 1) % vertices.len()].1) * (vertices[index].0 - vertices[(index + 1) % vertices.len()].0)) as f32);
    }

    return 0.5f32 * terms.iter().sum::<f32>();
}