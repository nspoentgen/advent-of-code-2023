use GridItem::*;
use RelativePosition::*;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum GridItem {
    Vertical,
    Horizontal,
    RightAngleNorthEast,
    RightAngleNorthWest,
    RightAngleSouthWest,
    RightAngleSouthEast,
    Ground,
    StartingPosition
}

impl GridItem {
    pub fn parse(symbol: char) -> Self {
        return match symbol {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => RightAngleNorthEast,
            'J' => RightAngleNorthWest,
            '7' => RightAngleSouthWest,
            'F' => RightAngleSouthEast,
            '.' => Ground,
            'S' => StartingPosition,
            _ => panic!("Invalid symbol")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum RelativePosition {
    North,
    East,
    South,
    West
}

//With convention that the normal points to interior of the pipe area
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum NormalDirection {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    Unknown
}

//positions = (row, col)
pub fn get_relative_position(source: &(usize, usize), dest: &(usize, usize)) -> RelativePosition {
    //Reversed for row direction because we are using matrix coordinate conventions
    return if dest.0 < source.0 {
        North
    } else if dest.0 > source.0 {
        South
    } else if dest.1 < source.1 {
        West
    } else {
        East
    }
}

lazy_static! {
    pub static ref VALID_JOIN_COMINATIONS: HashSet <(&'static GridItem, &'static GridItem, &'static RelativePosition)> = HashSet::from([
        (&Vertical, &Vertical, &North),
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
}

lazy_static! {
    pub static ref NORMAL_LOOKUP_TABLE: HashMap<(&'static GridItem, &'static GridItem, &'static NormalDirection), &'static NormalDirection> = HashMap::from([
        ((&Vertical, &Vertical, &NormalDirection::West), &NormalDirection::West),
        ((&Vertical, &Vertical, &NormalDirection::East), &NormalDirection::East),
        ((&Vertical, &RightAngleNorthEast, &NormalDirection::West), &NormalDirection::Southwest),
        ((&Vertical, &RightAngleNorthEast, &NormalDirection::East), &NormalDirection::Northeast),
        ((&Vertical, &RightAngleSouthEast, &NormalDirection::West), &NormalDirection::Northwest),
        ((&Vertical, &RightAngleSouthEast, &NormalDirection::East), &NormalDirection::Southeast),
        ((&Vertical, &RightAngleSouthWest, &NormalDirection::West), &NormalDirection::Southwest),
        ((&Vertical, &RightAngleSouthWest, &NormalDirection::East), &NormalDirection::Northeast),
        ((&Vertical, &RightAngleNorthWest, &NormalDirection::West), &NormalDirection::Northwest),
        ((&Vertical, &RightAngleNorthWest, &NormalDirection::East), &NormalDirection::Southeast),

        ((&Horizontal, &Horizontal, &NormalDirection::North), &NormalDirection::North),
        ((&Horizontal, &Horizontal, &NormalDirection::South), &NormalDirection::South),
        ((&Horizontal, &RightAngleNorthEast, &NormalDirection::North), &NormalDirection::Northeast),
        ((&Horizontal, &RightAngleNorthEast, &NormalDirection::South), &NormalDirection::Southwest),
        ((&Horizontal, &RightAngleSouthEast, &NormalDirection::North), &NormalDirection::Northwest),
        ((&Horizontal, &RightAngleSouthEast, &NormalDirection::South), &NormalDirection::Southeast),
        ((&Horizontal, &RightAngleSouthWest, &NormalDirection::North), &NormalDirection::Northeast),
        ((&Horizontal, &RightAngleSouthWest, &NormalDirection::South), &NormalDirection::Southwest),
        ((&Horizontal, &RightAngleNorthWest, &NormalDirection::North), &NormalDirection::Northwest),
        ((&Horizontal, &RightAngleNorthWest, &NormalDirection::South), &NormalDirection::Southeast),

        ((&RightAngleNorthEast, &Horizontal, &NormalDirection::Northeast), &NormalDirection::North),
        ((&RightAngleNorthEast, &Horizontal, &NormalDirection::Southwest), &NormalDirection::South),
        ((&RightAngleNorthEast, &Vertical, &NormalDirection::Northeast), &NormalDirection::East),
        ((&RightAngleNorthEast, &Vertical, &NormalDirection::Southwest), &NormalDirection::West),
        ((&RightAngleNorthEast, &RightAngleSouthEast, &NormalDirection::Northeast), &NormalDirection::Southeast),
        ((&RightAngleNorthEast, &RightAngleSouthEast, &NormalDirection::Southwest), &NormalDirection::Northwest),
        ((&RightAngleNorthEast, &RightAngleSouthWest, &NormalDirection::Northeast), &NormalDirection::Northeast),
        ((&RightAngleNorthEast, &RightAngleSouthWest, &NormalDirection::Southwest), &NormalDirection::Southwest),
        ((&RightAngleNorthEast, &RightAngleNorthWest, &NormalDirection::Northeast), &NormalDirection::Northwest),
        ((&RightAngleNorthEast, &RightAngleNorthWest, &NormalDirection::Southwest), &NormalDirection::Southeast),

        ((&RightAngleSouthEast, &Horizontal, &NormalDirection::Southeast), &NormalDirection::South),
        ((&RightAngleSouthEast, &Horizontal, &NormalDirection::Northwest), &NormalDirection::North),
        ((&RightAngleSouthEast, &Vertical, &NormalDirection::Southeast), &NormalDirection::East),
        ((&RightAngleSouthEast, &Vertical, &NormalDirection::Northwest), &NormalDirection::West),
        ((&RightAngleSouthEast, &RightAngleNorthEast, &NormalDirection::Southeast), &NormalDirection::Northeast),
        ((&RightAngleSouthEast, &RightAngleNorthEast, &NormalDirection::Northwest), &NormalDirection::Southwest),
        ((&RightAngleSouthEast, &RightAngleSouthWest, &NormalDirection::Southeast), &NormalDirection::Southwest),
        ((&RightAngleSouthEast, &RightAngleSouthWest, &NormalDirection::Northwest), &NormalDirection::Northeast),
        ((&RightAngleSouthEast, &RightAngleNorthWest, &NormalDirection::Southeast), &NormalDirection::Southeast),
        ((&RightAngleSouthEast, &RightAngleNorthWest, &NormalDirection::Northwest), &NormalDirection::Northwest),

        ((&RightAngleSouthWest, &Horizontal, &NormalDirection::Southwest), &NormalDirection::South),
        ((&RightAngleSouthWest, &Horizontal, &NormalDirection::Northeast), &NormalDirection::North),
        ((&RightAngleSouthWest, &Vertical, &NormalDirection::Southwest), &NormalDirection::West),
        ((&RightAngleSouthWest, &Vertical, &NormalDirection::Northeast), &NormalDirection::East),
        ((&RightAngleSouthWest, &RightAngleNorthEast, &NormalDirection::Southwest), &NormalDirection::Southwest),
        ((&RightAngleSouthWest, &RightAngleNorthEast, &NormalDirection::Northeast), &NormalDirection::Northeast),
        ((&RightAngleSouthWest, &RightAngleSouthEast, &NormalDirection::Southwest), &NormalDirection::Southeast),
        ((&RightAngleSouthWest, &RightAngleSouthEast, &NormalDirection::Northeast), &NormalDirection::Northwest),
        ((&RightAngleSouthWest, &RightAngleNorthWest, &NormalDirection::Southwest), &NormalDirection::Northwest),
        ((&RightAngleSouthWest, &RightAngleNorthWest, &NormalDirection::Northeast), &NormalDirection::Southeast),

        ((&RightAngleNorthWest, &Horizontal, &NormalDirection::Northwest), &NormalDirection::North),
        ((&RightAngleNorthWest, &Horizontal, &NormalDirection::Southeast), &NormalDirection::South),
        ((&RightAngleNorthWest, &Vertical, &NormalDirection::Northwest), &NormalDirection::West),
        ((&RightAngleNorthWest, &Vertical, &NormalDirection::Southeast), &NormalDirection::East),
        ((&RightAngleNorthWest, &RightAngleNorthEast, &NormalDirection::Northwest), &NormalDirection::Northeast),
        ((&RightAngleNorthWest, &RightAngleNorthEast, &NormalDirection::Southeast), &NormalDirection::Southwest),
        ((&RightAngleNorthWest, &RightAngleSouthEast, &NormalDirection::Northwest), &NormalDirection::Northwest),
        ((&RightAngleNorthWest, &RightAngleSouthEast, &NormalDirection::Southeast), &NormalDirection::Southeast),
        ((&RightAngleNorthWest, &RightAngleSouthWest, &NormalDirection::Northwest), &NormalDirection::Southwest),
        ((&RightAngleNorthWest, &RightAngleSouthWest, &NormalDirection::Southeast), &NormalDirection::Northeast),
        ]);
}
