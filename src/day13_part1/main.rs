use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use itertools::Itertools;
use std::path::Path;
use std::io::BufRead;
use num_format::{Locale, ToFormattedString};

type Chunk = Vec<Vec<char>>;

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
    const HORIZONTAL_SYMMETRY_ANSWER_COEFFICIENT: usize = 100;
    let vertical_symmetry_index = find_vertical_line_of_symmetry(chunk);
    let mut horizontal_symmetry_index = None;

    if vertical_symmetry_index.is_none() {
        horizontal_symmetry_index = find_horizontal_line_of_symmetry(chunk);
    }

    if vertical_symmetry_index.is_some() && horizontal_symmetry_index.is_some() {
        panic!("Something is wrong. There are two lines of symmetry in the chunk");
    }

    return if let Some(index) = vertical_symmetry_index {
        index
    } else if let Some(index) = horizontal_symmetry_index {
        HORIZONTAL_SYMMETRY_ANSWER_COEFFICIENT * index
    } else {
       panic!("Something is wrong. There are no lines of symmetry in chunk {}", chunk_index);
    };
}

fn find_vertical_line_of_symmetry(chunk: &Chunk) -> Option<usize> {
    let initial_range = (1usize..chunk[0].len())
        .collect::<Vec<usize>>();
    let mut possible_indices = HashSet::<usize>::from_iter(initial_range);

    for row_index in 0..chunk.len() {
        find_symmetry_index(chunk[row_index].clone(), &mut possible_indices);
    }

    return extract_answer_or_panic(&possible_indices,"Something is wrong. There is more than one vertical line of symmetry");
}

fn find_horizontal_line_of_symmetry(chunk: &Chunk) -> Option<usize> {
    let initial_range = (1usize..chunk.len())
        .collect::<Vec<usize>>();
    let mut possible_indices = HashSet::<usize>::from_iter(initial_range);

    //Rectangular size so choose first row arbitrarily for max column index
    for col_index in 0..chunk[0].len() {
        let test_col = chunk
            .iter()
            .map(|x| x[col_index])
            .collect::<Vec<char>>();

        find_symmetry_index(test_col, &mut possible_indices);
    }

    return extract_answer_or_panic(&possible_indices,"Something is wrong. There is more than one horizontal line of symmetry");
}

fn extract_answer_or_panic(answer_set: &HashSet<usize>, panic_message: &str) -> Option<usize> {
    return if answer_set.iter().count() == 1 {
        Some(*answer_set.iter().take(1).nth(0).unwrap())
    } else if answer_set.iter().count() == 0 {
        None
    } else {
        panic!("{}", panic_message);
    };
}

fn find_symmetry_index(line: Vec<char>, possible_indices: &mut HashSet<usize>) {
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