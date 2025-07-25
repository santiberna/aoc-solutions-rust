use crate::check_result;

fn calculate_elf1(input: usize) -> usize {
    let smallest_pow = input.checked_next_power_of_two().unwrap_or(0) / 2;
    let diff = input - smallest_pow;
    diff * 2 + 1
}

fn least_power_of_3(n: usize) -> usize {
    let mut v = 1;

    while v < n {
        v *= 3;
    }

    v / 3
}

fn calculate_elf2(input: usize) -> usize {
    let min = least_power_of_3(input);
    let diff = input - min;

    if diff <= min { diff } else { diff * 2 - min }
}

fn challenge(input: usize) -> (usize, usize) {
    (calculate_elf1(input), calculate_elf2(input))
}

check_result!(3004953, 1815603, 1410630);
