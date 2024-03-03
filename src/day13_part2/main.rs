use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::BufRead;
use num_format::{Locale, ToFormattedString};

mod chunk_solver;
use crate::chunk_solver::*;

fn main() {
    //Parse data
    let path = Path::new("src/day13_part1/input.txt");
    let all_data = parse_data(&path);

    //Find lines of symmetry and calculate answer
    let answer = all_data
        .iter()
        .enumerate()
        .map(|(index, chunk)| calculate_chunk_answer(index, chunk))
        .sum::<usize>();

    println!("The answer is {}", answer.to_formatted_string(&Locale::en))
}

fn parse_data(path: &Path) -> Vec<Chunk> {
    let mut all_chunks = Vec::<Chunk>::new();
    let mut chunk_buffer = Chunk::new();

    let file = File::open(&path).unwrap();
    for line in BufReader::new(file).lines().flatten() {
        if line.len() > 0 {
            chunk_buffer.push(line.chars().collect::<Vec<char>>());
        } else {
            all_chunks.push(chunk_buffer);
            chunk_buffer = Chunk::new();
        }
    }

    //Edge case for final chunk
    all_chunks.push(chunk_buffer);

    return all_chunks;
}

fn calculate_chunk_answer(chunk_index: usize, chunk: &Chunk) -> usize {
    let original_solver = ChunkSolver { chunk: chunk.clone(), remove_option: None};
    let original_answer = original_solver.solve_chunk().unwrap();

    for i in 0..chunk.len() {
        for j in 0..chunk[i].len() {
            let mut test_chunk = chunk.clone();
            if test_chunk[i][j] == ASH {
                test_chunk[i][j] = ROCK;
            } else {
                test_chunk[i][j] = ASH;
            }

            let remove_option = (original_answer.0, original_answer.1);
            let chunk_solver = ChunkSolver { chunk: test_chunk, remove_option: Some(remove_option) };

            if let Some(possible_answer) = chunk_solver.solve_chunk() {
                return possible_answer.2;
            }
        }
    }

    panic!("No answer for chunk {}", chunk_index);
}
