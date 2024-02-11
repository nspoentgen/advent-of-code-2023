use std::cmp::min;

//Range as in math domain & range.
pub fn get_output_ranges(input: &(u64, u64), map_list: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut output_ranges = Vec::<(u64, u64)>::new();
    let mut domain_cursor = input.0;
    let mut num_elements_remaining = input.1;

    while num_elements_remaining > 0 {
        let subrange_start = get_mapped_output(domain_cursor, &map_list);
        let subdomain_end;

        match find_map_index(domain_cursor, &map_list) {
            Some(map_index) => {
                subdomain_end = min(domain_cursor + num_elements_remaining - 1, map_list[map_index].0 + map_list[map_index].2 - 1);
            },
            None => {
                let map_min = map_list
                    .iter()
                    .filter(|x| x.1 > domain_cursor)
                    .map(|x| x.1)
                    .min().unwrap_or_else(|| u64::MAX);
                subdomain_end = min(domain_cursor + num_elements_remaining - 1, map_min);
            }
        }

        output_ranges.push((subrange_start, subdomain_end - domain_cursor + 1));
        num_elements_remaining -= subdomain_end - domain_cursor + 1;
        domain_cursor = subdomain_end + 1;
    }

    return output_ranges;
}

pub fn get_mapped_output(input: u64, map_list: &Vec<(u64, u64, u64)>) -> u64 {
    match find_map_index(input, &map_list) {
        Some(map_index) => {
            let map_data = map_list[map_index];
            apply_map(input, map_data.0, map_data.1, map_data.2)
        },
        None => input
    }
}

pub fn find_map_index(input: u64, map_list: &Vec<(u64, u64, u64)>) -> Option<usize> {
    for (index, map) in map_list.iter().enumerate() {
        if input >= map.0 && input < map.0 + map.2 {
            return Some(index);
        }
    }

    return None;
}

pub fn apply_map(x: u64, x0: u64, y0: u64, length: u64) -> u64 {
    if x >= x0 + length {
        panic!("Can't map");
    }

    return y0 + (x - x0);
}
