use std::collections::HashMap;

use crate::check_result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VAL_REGEX: Regex = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    static ref GIVE_REGEX: Regex =
        Regex::new(r"^bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)$")
            .unwrap();
}

enum Target {
    Robot(usize),
    Output(usize),
}

type Node = (Target, Target);

type State = [Vec<usize>; 256];

fn setup_init(mut acc: Box<State>, s: &str) -> Box<State> {
    if let Some(captures) = VAL_REGEX.captures(s) {
        let value: usize = captures[1].parse().unwrap();
        let robot: usize = captures[2].parse().unwrap();

        acc[robot].push(value);
        assert!(acc[robot].len() <= 2);
    }

    acc
}

fn setup_graph(mut acc: HashMap<usize, Node>, s: &str) -> HashMap<usize, Node> {
    if let Some(captures) = GIVE_REGEX.captures(s) {
        let from: usize = captures[1].parse().unwrap();
        let low_v: usize = captures[3].parse().unwrap();
        let high_v: usize = captures[5].parse().unwrap();

        let low = match &captures[2] {
            "bot" => Target::Robot(low_v),
            "output" => Target::Output(low_v),
            _ => panic!(),
        };

        let high = match &captures[4] {
            "bot" => Target::Robot(high_v),
            "output" => Target::Output(high_v),
            _ => panic!(),
        };

        acc.insert(from, (low, high)).is_none();
    }

    acc
}

fn get_values((i, v): (usize, &Vec<usize>)) -> (usize, usize, usize) {
    (i, *v.iter().min().unwrap(), *v.iter().max().unwrap())
}

fn challenge(input: &str) -> (usize, usize) {
    let text: String = std::fs::read_to_string(input).unwrap();
    let graph = text.lines().fold(HashMap::new(), setup_graph);

    let initial = Box::new(std::array::from_fn(|_| Vec::new()));

    let mut state = text.lines().fold(initial, setup_init);
    let mut special_robot = 0usize;
    let mut outputs: [Option<usize>; 32] = [None; 32];

    while let Some((robot_id, min, max)) = state
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 2)
        .map(get_values)
    {
        let (low_target, high_target) = graph.get(&robot_id).unwrap();

        if min == 17 && max == 61 {
            special_robot = robot_id;
        }

        match low_target {
            Target::Output(v) => outputs[*v] = Some(min),
            Target::Robot(v) => state[*v].push(min),
        }

        match high_target {
            Target::Output(v) => outputs[*v] = Some(max),
            Target::Robot(v) => state[*v].push(max),
        }

        state[robot_id].clear();
    }

    let product: usize = [outputs[0], outputs[1], outputs[2]]
        .into_iter()
        .flatten()
        .product();

    (special_robot, product)
}

check_result!("input/C10.txt", 118, 143153);
