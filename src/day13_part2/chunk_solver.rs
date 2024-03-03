use std::collections::HashSet;
use itertools::Itertools;
use crate::chunk_solver::Orientation::{Horizontal, Vertical};

pub type Chunk = Vec<Vec<char>>;
pub const ASH: char = '.';
pub const ROCK: char = '#';

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Orientation {Horizontal, Vertical}

pub struct ChunkSolver {
    pub chunk: Chunk,
    pub remove_option: Option<(usize, Orientation)>
}

impl ChunkSolver {
    pub fn solve_chunk(&self) -> Option<(usize, Orientation, usize)> {
        const HORIZONTAL_SYMMETRY_ANSWER_COEFFICIENT: usize = 100;
        let vertical_symmetry_index = self.find_vertical_line_of_symmetry();
        let horizontal_symmetry_index = self.find_horizontal_line_of_symmetry();

        return if vertical_symmetry_index.is_some() && horizontal_symmetry_index.is_some() ||
                  vertical_symmetry_index.is_none() && horizontal_symmetry_index.is_none()
        {
            None
        } else if let Some(index) = vertical_symmetry_index {
            Some((index, Vertical, index))
        } else if let Some(index) = horizontal_symmetry_index {
            Some((index, Horizontal, HORIZONTAL_SYMMETRY_ANSWER_COEFFICIENT * index))
        } else {
            panic!("Condition not mapped")
        };
    }

    fn find_vertical_line_of_symmetry(&self) -> Option<usize> {
        let initial_range = (1usize..self.chunk[0].len())
            .collect::<Vec<usize>>();
        let mut possible_indices = HashSet::<usize>::from_iter(initial_range);
        if let Some(remove_data) = &self.remove_option {
            if remove_data.1 == Vertical {
                possible_indices.remove(&remove_data.0);
            }
        }

        for row_index in 0..self.chunk.len() {
            self.find_symmetry_index(self.chunk[row_index].clone(), &mut possible_indices);
        }

        return self.extract_answer(&possible_indices);
    }

    fn find_horizontal_line_of_symmetry(&self) -> Option<usize> {
        let initial_range = (1usize..self.chunk.len())
            .collect::<Vec<usize>>();
        let mut possible_indices = HashSet::<usize>::from_iter(initial_range);
        if let Some(remove_data) = &self.remove_option {
            if remove_data.1 == Horizontal {
                possible_indices.remove(&remove_data.0);
            }
        }

        //Rectangular size so choose first row arbitrarily for max column index
        for col_index in 0..self.chunk[0].len() {
            let test_col = self.chunk
                .iter()
                .map(|x| x[col_index])
                .collect::<Vec<char>>();

            self.find_symmetry_index(test_col, &mut possible_indices);
        }

        return self.extract_answer(&possible_indices);
    }

    fn extract_answer(&self, answer_set: &HashSet<usize>) -> Option<usize> {
        return if answer_set.iter().count() == 1 {
            Some(*answer_set.iter().take(1).nth(0).unwrap())
        } else {
            None
        };
    }

    fn find_symmetry_index(&self, line: Vec<char>, possible_indices: &mut HashSet<usize>) {
        //Line search init
        let max_index: isize = (line.len() - 1) as isize;
        let mut indices_to_remove = Vec::<usize>::new();

        for test_index in possible_indices.iter().sorted() {
            //Middle-out search init
            let mut right_cursor = *test_index as isize;
            let mut left_cursor = (*test_index - 1) as isize;
            let mut keep_searching = true;
            let mut is_symmetrical = true;
            let mut symmetry_length = 0isize;

            //Middle-out contiguous equality
            while keep_searching {
                is_symmetrical = line[left_cursor as usize] == line[right_cursor as usize];
                if is_symmetrical {
                    symmetry_length += 1;
                }

                left_cursor -= 1;
                right_cursor += 1;
                let in_bounds = left_cursor >= 0 && right_cursor <= max_index;
                keep_searching = is_symmetrical && in_bounds;
            }

            //Adjust the cursor so that they reflect the symmetric range
            //(the cursors can cross each other which means no symmetric range)
            if is_symmetrical {
                left_cursor += 1;
                right_cursor -= 1
            } else{
                left_cursor += 2;
                right_cursor -= 2
            }

            //If no symmetric range, or the symmetric range doesn't extend
            //to at least one end of the line, or the symmetric range isn't
            //bigger than the last range, then remove the index. Otherwise,
            //record the symmetry data and remove any previous symmetry index.
            if symmetry_length == 0 ||
                (left_cursor > 0 && right_cursor < max_index)
            {
                indices_to_remove.push(*test_index);
            }
        }

        //Remove invalid indices
        for index in indices_to_remove {
            possible_indices.remove(&index);
        }
    }
}