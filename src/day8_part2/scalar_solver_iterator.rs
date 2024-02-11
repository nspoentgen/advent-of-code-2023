use std::collections::HashMap;
use crate::step_options::StepOptions;

pub struct ScalarSolverIterator<'a> {
    node: &'a String,
    instructions: &'a String,
    desert_map: &'a HashMap<String, StepOptions>,
    num_steps: u64
}

#[derive(Hash, Eq, PartialOrd, PartialEq, Debug)]
pub struct ScalarSolverResult {
    pub index: usize,
    pub iteration: u64
}

impl<'a> ScalarSolverIterator<'a> {
    pub fn new(
        start_node: &'a String,
        instructions: &'a String,
        desert_map: &'a HashMap<String, StepOptions>) -> ScalarSolverIterator<'a> {
        ScalarSolverIterator {
            node: start_node,
            instructions,
            desert_map,
            num_steps: 0
        }
    }
}

impl<'a> Iterator for ScalarSolverIterator<'a> {
    type Item = ScalarSolverResult;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first_iteration = true;

        while first_iteration || !self.node.ends_with("Z") {
            let next_step = self.instructions
                .chars()
                .nth(self.num_steps as usize % self.instructions.len())
                .unwrap();

            self.node = match next_step {
                'L' => &self.desert_map[self.node].left,
                'R' => &self.desert_map[self.node].right,
                _ => panic!("Cannot map instruction")
            };

            self.num_steps += 1;
            first_iteration = false;
        }

        let solver_result = ScalarSolverResult{
            index: self.num_steps as usize % self.instructions.len(),
            iteration: self.num_steps / self.instructions.len() as u64
        };

        return Some(solver_result);
    }
}
