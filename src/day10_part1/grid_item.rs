use GridItem::*;
use RelativePosition::*;

#[derive(PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq, Hash)]
pub enum RelativePosition {
    North,
    East,
    South,
    West
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
