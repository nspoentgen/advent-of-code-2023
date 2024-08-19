use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use ndarray::stack;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use ndarray_linalg::error as nde;
use num_format::{Locale, ToFormattedString};

extern crate intel_mkl_src;

fn main() {
    let path = Path::new("src/day24_part1/test_input.txt");
    let data = parse_data(&path);

    let time_results = get_time_results(&data.0, &data.1);
    let intersection_results = get_intersection_results(data.0, data.1, time_results);
    let filtered_intersections = intersection_results
        .iter()
        .filter(|&v| v.0 >= 200000000000000.0 && v.0 <= 400000000000000.0 && v.1 >= 200000000000000.0 && v.1 <= 400000000000000.0)
        .collect_vec();

    println!("The number of intersections is {}", filtered_intersections.len().to_formatted_string(&Locale::en));
}

fn parse_data(path: &Path) -> (Vec<Array1<f64>>, Vec<Array1<f64>>) {
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
            let mut array = Array1::from_iter(num_iter).t().to_owned();
            array[2] = 0.0;
            return array;
        })
        .collect_vec();

    let velocities = line_splits
        .map(|line_split| {
            let num_iter = line_split[1].split(",").map(|num| num.trim().parse::<f64>().unwrap());
            let mut array = Array1::from_iter(num_iter).t().to_owned();
            array[2] = 0.0;
            return array;
        })
        .collect_vec();

    return (positions, velocities);
}

fn get_time_results(positions: &Vec<Array1<f64>>, velocities: &Vec<Array1<f64>>) -> Vec<nde::Result<Array1<f64>>> {
    let mut results = vec![];

    for i in 0..positions.len() - 1{
        for j in i + 1..positions.len() {
            results.push(solve_system(&positions[i], velocities[i].clone(), positions[j].clone(), &velocities[j]));
        }
    }

    return results;
}

fn get_intersection_results(positions: Vec<Array1<f64>>, velocities: Vec<Array1<f64>>, time_results: Vec<nde::Result<Array1<f64>>>, ) -> Vec<(f64, f64)> {
    return time_results
        .iter()
        .enumerate()
        .filter(|pair| pair.1.is_ok())
        .map(|pair| &positions[pair.0] + &velocities[pair.0] * (pair.1.as_ref().unwrap()[0]))
        .map(|x| (x[0], x[1]))
        .collect_vec();
}

fn solve_system(p1: &Array1<f64>, v1: Array1<f64>, p2: Array1<f64>, v2: &Array1<f64>) -> nde::Result<Array1<f64>> {
    //let b = p2 - p1;
    //let A = stack![Axis(1), v1, -v2];
    //println!("A = {:?}", A);
    //println!("b = {:?}", b);

    let A = array![[-2., 1.], [1., 1.], [1., 0.]];
    let b = array![-1.0, 6., 0.];
    let foo =A.solve(&b);
    return foo;
}