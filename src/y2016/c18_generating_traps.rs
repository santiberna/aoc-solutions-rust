fn determine_tile(left: bool, center: bool, right: bool) -> bool {
    (left && center && !right)
        || (!left && center && right)
        || (left && !center && !right)
        || (!left && !center && right)
}

fn gen_tile(prev_row: &[bool], i: usize) -> bool {
    let left = prev_row.get(i.wrapping_sub(1)).copied().unwrap_or(false);
    let center = prev_row.get(i).copied().unwrap_or(false);
    let right = prev_row.get(i + 1).copied().unwrap_or(false);
    determine_tile(left, center, right)
}

fn safe_tiles(input: &[bool], rows: usize) -> usize {
    let mut counter: usize = input.iter().map(|&b| !b as usize).sum();

    let mut prev_row = input.to_vec();
    let mut current_row = Vec::with_capacity(prev_row.capacity());

    for _ in 1..rows {
        current_row.clear();

        current_row.extend((0..prev_row.len()).map(|i| gen_tile(&prev_row, i)));
        counter += current_row.iter().map(|&b| !b as usize).sum::<usize>();

        std::mem::swap(&mut current_row, &mut prev_row);
    }

    counter
}

fn challenge(input: &str) -> (usize, usize) {
    let start_row: Vec<bool> = std::fs::read_to_string(input)
        .unwrap()
        .chars()
        .map(|c| if c == '^' { true } else { false })
        .collect();

    (safe_tiles(&start_row, 40), safe_tiles(&start_row, 400000))
}

//check_result!("input/Y2016/C18.txt", 1989, 19999894);
