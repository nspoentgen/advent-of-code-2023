use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

use nalgebra::{Const, Matrix2, Matrix3, OMatrix, Vector2, Vector3};

fn main() {
    let lower_bound = 200000000000000.0;
    let upper_bound = 400000000000000.0;

    let path = Path::new("src/day24_part1/input.txt");
    let data = parse_data(&path);
    let time_results = get_time_results(&data.0, &data.1);
    let intersection_results = get_intersection_results(data.0, data.1, time_results);
    let filtered_intersections = intersection_results
        .iter()
        .filter(|&v| v.0 >= lower_bound && v.0 <= upper_bound && v.1 >= lower_bound && v.1 <= upper_bound)
        .collect_vec();


    println!("The number of intersections is {}", filtered_intersections.len().to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> (Vec<Vector3<f64>>, Vec<Vector3<f64>>) {
    let file = File::open(&path).unwrap();

    let lines = BufReader::new(file)
        .lines()
        .flatten()
        .collect_vec();

    let line_splits = lines
        .iter()
        .map(|line| line.split("@").map(|s| s.to_string()).collect_vec());

    let positions = line_splits
        .clone()
        .map(|line_split| {
            let num_iter = line_split[0].split(",").map(|num| num.trim().parse::<f64>().unwrap());
            let mut array = Vector3::from_iterator(num_iter);
            return array;
        })
        .collect_vec();

    let velocities = line_splits
        .map(|line_split| {
            let num_iter = line_split[1].split(",").map(|num| num.trim().parse::<f64>().unwrap());
            let mut array = Vector3::from_iterator(num_iter);
            return array;
        })
        .collect_vec();

    return (positions, velocities);
}

fn get_time_results(positions: &Vec<Vector3<f64>>, velocities: &Vec<Vector3<f64>>) -> Vec<((usize, usize), Option<OMatrix<f64, Const<2>, Const<1>>>)> {
    let mut results = vec![];

    for i in 0..positions.len() - 1{
        for j in i + 1..positions.len() {
            results.push(((i, j), solve_system(&positions[i], velocities[i].clone(), positions[j].clone(), &velocities[j])));
        }
    }

    return results;
}

fn get_intersection_results(positions: Vec<Vector3<f64>>, velocities: Vec<Vector3<f64>>, time_results: Vec<((usize, usize), Option<OMatrix<f64, Const<2>, Const<1>>>)>) -> Vec<(f64, f64)> {
    return time_results
        .iter()
        .filter(|pair| pair.1.is_some())
        .filter(|pair| pair.1.unwrap()[0] >= 0.0 && pair.1.unwrap()[1] >= 0.0)
        .map(|pair| &positions[pair.0.0] + &velocities[pair.0.0] * (pair.1.as_ref().unwrap()[0]))
        .map(|x| (x[0], x[1]))
        .collect_vec();
}

fn solve_system(p1: &Vector3<f64>, v1: Vector3<f64>, p2: Vector3<f64>, v2: &Vector3<f64>) -> Option<OMatrix<f64, Const<2>, Const<1>>> {
    let b = Vector2::new(p2.x - p1.x, p2.y - p1.y );
    let A = Matrix2::new(v1.x, -v2.x, v1.y, -v2.y);
    let lu = A.lu();
    return lu.solve(&b);
}