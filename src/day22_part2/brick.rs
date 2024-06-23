use std::collections::HashMap;
use crate::Coordinate;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum Orientation {
    X, Y, Z
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Brick {
    pub id: u32,
    pub start_position: Coordinate,
    pub end_position: Coordinate,
    pub orientation: Orientation
}



impl Brick {
    pub fn new(id: u32, start_position: Coordinate, end_position: Coordinate) -> Self {
        let orientation = if start_position.0 != end_position.0 {
            Orientation::X
        } else if start_position.1 != end_position.1 {
            Orientation::Y
        } else {
            Orientation::Z
        };

        return Brick { id, start_position, end_position, orientation};
    }

    pub fn can_fall_one(&self, world_map: &mut HashMap<Coordinate, u32>) -> bool {
        if self.start_position.2 == 1 || self.end_position.2 == 1 {
            return false;
        }

        match self.orientation {
            Orientation::X => {
                for x in self.start_position.0..=self.end_position.0 {
                    if world_map.contains_key(&(x, self.start_position.1, self.start_position.2 - 1)) {
                        return false;
                    }
                }
            },
            Orientation::Y => {
                for y in self.start_position.1..=self.end_position.1 {
                    if world_map.contains_key(&(self.start_position.0, y, self.start_position.2 - 1)) {
                        return false;
                    }
                }
            },
            Orientation::Z => {
                if world_map.contains_key(&(self.start_position.0, self.start_position.1, self.start_position.2 - 1)) {
                    return false;
                }
            },
        };

        return true;
    }

    pub fn fall(&mut self, amount: u64, world_map: &mut HashMap<Coordinate, u32>) {
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