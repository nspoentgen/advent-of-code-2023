use std::ops;
use crate::grid_data::PerimeterMovement::*;

pub const SQUARE_EDGE_LENGTH: f32 = 1f32;
pub type DiscreteCoordinateInt = (i32, i32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PerimeterMovement {
    North,
    NorthToEast,
    East,
    EastToSouth,
    South,
    SouthToWest,
    West,
    WestToNorth,
    SouthToEast,
    EastToNorth,
    NorthToWest,
    WestToSouth,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    CW,
    CCW
}

#[derive(Clone)]
pub struct DiscreteCoordinateFloat {
    pub point: (f32, f32)
}

impl DiscreteCoordinateFloat {
    //Rounding error will occur if the coordinates are not on integer boundaries
    pub fn round_to_discrete_coordinate_int(&self) -> DiscreteCoordinateInt {
        return (self.point.0.round() as i32, self.point.1.round() as i32);
    }

    pub fn x_eq(&self, other:&Self) -> bool {
        return float_equals(self.point.0, other.point.0);
    }

    pub fn y_eq(&self, other:&Self) -> bool {
        return float_equals(self.point.1, other.point.1);
    }
}

impl PartialEq<Self> for DiscreteCoordinateFloat {
    fn eq(&self, other: &Self) -> bool {
        return self.x_eq(other) && self.y_eq(other);
    }
}

impl Eq for DiscreteCoordinateFloat {}

impl ops::Add for DiscreteCoordinateFloat {
    type Output = DiscreteCoordinateFloat;

    fn add(self, rhs: Self) -> Self::Output {
        return DiscreteCoordinateFloat { point: (self.point.0 + rhs.point.0, self.point.1 + rhs.point.1)};
    }
}

impl ops::Add<DiscreteCoordinateFloat> for &DiscreteCoordinateFloat {
    type Output = DiscreteCoordinateFloat;

    fn add(self, rhs: DiscreteCoordinateFloat) -> Self::Output {
        return DiscreteCoordinateFloat { point: (self.point.0 + rhs.point.0, self.point.1 + rhs.point.1)};
    }
}

pub fn get_perimeter_movement(n_minus_one: &DiscreteCoordinateFloat, n: &DiscreteCoordinateFloat, n_plus_one: &DiscreteCoordinateFloat) -> PerimeterMovement {
    if n.x_eq(n_minus_one) && n.x_eq(n_plus_one) && n_plus_one.point.1 > n.point.1 {
        return North;
    }

    if n.x_eq(n_minus_one) && n.point.1 > n_minus_one.point.1 && n_plus_one.point.0 > n.point.0 {
        return NorthToEast;
    }

    if n.y_eq(n_minus_one) && n.y_eq(n_plus_one) && n_plus_one.point.0 > n.point.0 {
        return East
    }

    if n.y_eq(n_minus_one) && n.point.0 > n_minus_one.point.0 && n_plus_one.point.1 < n.point.1 {
        return EastToSouth;
    }

    if n.x_eq(n_minus_one) && n.x_eq(n_plus_one) && n_plus_one.point.1 < n.point.1 {
        return South;
    }

    if n.x_eq(n_minus_one) && n.point.1 < n_minus_one.point.1 && n_plus_one.point.0 < n.point.0 {
        return SouthToWest;
    }

    if n.y_eq(n_minus_one) && n.y_eq(n_plus_one) && n_plus_one.point.0 < n.point.0 {
        return West
    }

    if n.y_eq(n_minus_one) && n.point.0 < n_minus_one.point.0 && n_plus_one.point.1 > n.point.1 {
        return WestToNorth;
    }

    if n.x_eq(n_minus_one) && n.point.1 < n_minus_one.point.1 && n_plus_one.point.0 > n.point.0 {
        return SouthToEast;
    }

    if n.y_eq(n_minus_one) && n.point.0 > n_minus_one.point.0 && n_plus_one.point.1 > n.point.1 {
        return EastToNorth;
    }

    if n.x_eq(n_minus_one) && n.point.1 > n_minus_one.point.1 && n_plus_one.point.0 < n.point.0 {
        return NorthToWest;
    }

    if n.y_eq(n_minus_one) && n.point.0 < n_minus_one.point.0 && n_plus_one.point.1 < n.point.1 {
        return WestToSouth;
    }

    panic!("Condition not mapped to perimeter movement");
}

fn float_equals(lhs: f32, rhs: f32) -> bool {
    return f32::abs(lhs - rhs) <= 1e-6;
}