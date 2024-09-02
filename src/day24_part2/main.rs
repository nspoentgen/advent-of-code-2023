use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;
use num_format::{Locale, ToFormattedString};

use nalgebra::{Const, Matrix2, Matrix3, Matrix6, OMatrix, Vector2, Vector3, Vector6};

fn main() {
    let path = Path::new("src/day24_part1/input.txt");
    let data = parse_data(&path);
    let (p, v) = solve_rock_data(&data.0[0], &data.1[0], &data.0[1], &data.1[1], &data.0[2], &data.1[2]);

    println!("p sum = {}", p.sum());
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

fn solve_system(ap: &Vector3<f64>, av: &Vector3<f64>, bp: &Vector3<f64>, bv: &Vector3<f64>, cp: &Vector3<f64>, cv: &Vector3<f64>) -> Option<OMatrix<f64, Const<6>, Const<1>>> {
    let b = Vector6::<f64>::new(
        ap.y*av.z - bp.y*bv.z - ap.z*av.y + bp.z*bv.y,
        -ap.x*av.z + bp.x*bv.z - ap.z*av.x + bp.z*bv.x,
        ap.x*av.y - bp.x*bv.y - ap.y*av.x + bp.y*bv.x,
        ap.y*av.z - cp.y*cv.z - ap.z*av.y + cp.z*cv.y,
        -ap.x*av.z + cp.x*cv.z - ap.z*av.x + cp.z*cv.x,
        ap.x*av.y - cp.x*cv.y - ap.y*av.x + cp.y*cv.x,
    );
    
    let A = Matrix6::<f64>::new(
        0f64, av.z - bv.z, -av.y+bv.y, 0f64, -ap.z+bp.z, ap.y-bp.y,
        -av.z+bv.z, 0f64, av.x - bv.x, ap.z - bp.z, 0f64, -ap.x + bp.x,
        av.y - bv.y, -av.x + bv.x, 0f64, -ap.y + bp.y, ap.x - bp.x, 0f64,
        0f64, av.z - cv.z, -av.y+cv.y, 0f64, -ap.z+cp.z, ap.y-cp.y,
        -av.z+cv.z, 0f64, av.x - cv.x, ap.z - cp.z, 0f64, -ap.x + cp.x,
        av.y - cv.y, -av.x + cv.x, 0f64, -ap.y + cp.y, ap.x - cp.x, 0f64
    );
    let lu = A.lu();
    return lu.solve(&b);
}

fn solve_rock_data(hp0: &Vector3<f64>, hv0: &Vector3<f64>, hp1: &Vector3<f64>, hv1: &Vector3<f64>, hp2: &Vector3<f64>, hv2: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let p1 = hp1 - hp0;
    let p2 = hp2 - hp0;;
    let v1 = hv1 - hv0;
    let v2 = hv2 - hv0;

    let t1 = -(p1.cross(&p2).dot(&v2)) / (v1.cross(&p2).dot(&v2));
    let t2 = -(p1.cross(&p2).dot(&v1)) / (p1.cross(&v2).dot(&v1));

    let c1 = hp1 + t1 * hv1;
    let c2 = hp2 + t2 * hv2;

    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - t1*v;

    return (p, v);
}