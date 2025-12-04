use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    static ref CHIP_REGEX: Regex = Regex::new(r"(\w+)-compatible microchip").unwrap();
    static ref GEN_REGEX: Regex = Regex::new(r"(\w+) generator").unwrap();
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
struct State {
    elevator: usize,
    items: Vec<(usize, usize)>,
}

fn parse_initial_state(s: &str) -> State {
    let mut out = State::default();

    let name_indices: HashMap<String, usize> = CHIP_REGEX
        .captures_iter(s)
        .map(|caps| caps[1].to_string())
        .collect::<HashSet<String>>() // remove duplicates
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    out.items.resize(name_indices.len(), (42, 42));

    for (floor, line) in s.lines().enumerate() {
        for m in CHIP_REGEX.captures_iter(line) {
            let index = name_indices.get(&m[1]).copied().unwrap();
            out.items[index].0 = floor;
        }

        for m in GEN_REGEX.captures_iter(line) {
            let index = name_indices.get(&m[1]).copied().unwrap();
            out.items[index].1 = floor;
        }
    }

    out
}

fn is_goal(s: &State) -> bool {
    s.items.iter().all(|&(a, b)| a == 3 && b == 3)
}

fn is_valid(state: &State) -> bool {
    for floor in 0..4 {
        let gens: Vec<_> = state.items.iter().filter(|(_, g)| *g == floor).collect();
        if gens.is_empty() {
            continue;
        }

        for &(chip, generator) in &state.items {
            if chip == floor && generator != floor {
                return false;
            }
        }
    }
    true
}

fn normalize(state: &State) -> State {
    let mut items = state.items.clone();
    items.sort_unstable(); // avoid equivalent permutations
    State {
        elevator: state.elevator,
        items,
    }
}

#[derive(Clone, Copy)]
enum Item {
    Gen(usize),
    Chip(usize),
}

fn next_states(current: &State) -> Vec<State> {
    let mut result = vec![];
    let floor = current.elevator;

    // Get all items in the current floor

    let mut indices = vec![];
    for (i, &(chip, generator)) in current.items.iter().enumerate() {
        if chip == floor {
            indices.push(Item::Chip(i));
        }
        if generator == floor {
            indices.push(Item::Gen(i));
        }
    }

    // All combinations of 1 or 2 items

    let mut combos = vec![];

    for i in 0..indices.len() {
        combos.push(vec![indices[i]]);
        for j in i + 1..indices.len() {
            combos.push(vec![indices[i], indices[j]]);
        }
    }

    for &dir in &[-1i32, 1] {
        let new_floor = (floor as i32 + dir) as usize;
        if new_floor > 3 {
            continue;
        }

        for combo in &combos {
            let mut new_state = current.clone();
            new_state.elevator = new_floor;

            for item in combo {
                match item {
                    &Item::Chip(i) => new_state.items[i].0 = new_floor,
                    &Item::Gen(i) => new_state.items[i].1 = new_floor,
                }
            }

            if is_valid(&new_state) {
                result.push(new_state);
            }
        }
    }

    result
}

fn bfs(initial: &State) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(normalize(initial));
    queue.push_back((initial.clone(), 0));

    while let Some((state, steps)) = queue.pop_front() {
        if is_goal(&state) {
            return steps;
        }

        let next_states = next_states(&state);

        for next in next_states.into_iter() {
            if visited.insert(normalize(&next)) {
                queue.push_back((next, steps + 1));
            }
        }
    }

    unreachable!("Should always find a solution");
}

fn challenge(input: &str) -> (usize, usize) {
    let mut state = parse_initial_state(&std::fs::read_to_string(input).unwrap());

    let part1 = bfs(&state);

    state.items.push((0, 0));
    state.items.push((0, 0));

    let part2 = bfs(&state);

    (part1, part2)
}

// use crate::check_result;
//check_result!("input/Y2016/C11.txt", 37, 0);
