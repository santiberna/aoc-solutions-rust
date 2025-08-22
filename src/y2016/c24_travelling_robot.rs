use std::collections::{BTreeMap, HashSet, VecDeque};

use nalgebra::Vector2;

use crate::{check_result, utility::MatrixVec};

type Vec2 = Vector2<i64>;

fn parse_map(input: &str) -> MatrixVec<u8> {
    let file = std::fs::read_to_string(input).unwrap();
    let height = file.lines().count();

    let data: Vec<u8> = file
        .lines()
        .flat_map(|s| s.bytes().collect::<Vec<u8>>())
        .collect();

    MatrixVec::<u8>::from_vec(height, data.len() / height, data)
}

fn find_check_points(map: &MatrixVec<u8>) -> BTreeMap<i64, Vec2> {
    let mut out = BTreeMap::new();

    for y in 0..map.rows() {
        for x in 0..map.cols() {
            let num = *map.get(y, x).unwrap();
            if num.is_ascii_digit() {
                out.insert((num - b'0') as i64, Vec2::new(x as i64, y as i64));
            }
        }
    }

    out
}

const DIRECTIONS: [Vec2; 4] = [
    Vec2::new(1, 0),
    Vec2::new(-1, 0),
    Vec2::new(0, -1),
    Vec2::new(0, 1),
];

fn flood_fill_find_points(map: &MatrixVec<u8>, start: Vec2) -> BTreeMap<i64, i64> {
    let mut map_copy = map.clone();
    let mut out = BTreeMap::new();
    let mut queue: VecDeque<(Vec2, i64)> = VecDeque::new();

    queue.push_back((start, 0));
    map_copy.set(start.y as usize, start.x as usize, b'#');

    while let Some(next) = queue.pop_front() {
        map_copy.set(next.0.y as usize, next.0.x as usize, b'#');

        for dir in DIRECTIONS {
            let t = next.0 + dir;
            let c = next.1 + 1;

            let tile = *map_copy.get(t.y as usize, t.x as usize).unwrap();

            if tile.is_ascii_digit() {
                out.insert((tile - b'0') as i64, c);
            }

            if tile != b'#' {
                map_copy.set(t.y as usize, t.x as usize, b'#');
                queue.push_back((t, c));
            }
        }
    }

    out
}

fn build_graph(map: &MatrixVec<u8>, points: &BTreeMap<i64, Vec2>) -> MatrixVec<i64> {
    let check_points_count = *points.last_key_value().unwrap().0 as usize + 1;
    let mut graph = MatrixVec::<i64>::new(check_points_count, check_points_count);

    for (&row, position) in points {
        let found_paths = flood_fill_find_points(map, *position);

        for (col, weight) in found_paths {
            graph.set(row as usize, col as usize, weight);
        }
    }

    graph
}

fn find_shortest(graph: &MatrixVec<i64>, current: usize, unvisited: HashSet<usize>) -> i64 {
    if unvisited.is_empty() {
        return 0;
    }

    let mut result = i64::MAX;

    for &node in &unvisited {
        let cost = graph.get(current, node).unwrap();
        let mut set = unvisited.clone();
        set.remove(&node);

        result = result.min(cost + find_shortest(graph, node, set));
    }

    result
}

fn find_shortest2(graph: &MatrixVec<i64>, current: usize, unvisited: HashSet<usize>) -> i64 {
    if unvisited.is_empty() {
        return *graph.get(current, 0).unwrap();
    }

    let mut result = i64::MAX;

    for &node in &unvisited {
        let cost = graph.get(current, node).unwrap();
        let mut set = unvisited.clone();
        set.remove(&node);

        result = result.min(cost + find_shortest2(graph, node, set));
    }

    result
}

fn challenge(input: &str) -> (i64, i64) {
    let map = parse_map(input);
    let points = find_check_points(&map);

    let graph = build_graph(&map, &points);

    let node_count = (points.last_key_value().unwrap().0 + 1) as usize;
    let unvisited: HashSet<usize> = (1..node_count).collect();

    (
        find_shortest(&graph, 0, unvisited.clone()),
        find_shortest2(&graph, 0, unvisited),
    )
}

check_result!("input/Y2016/C24.txt", 474, 696);
