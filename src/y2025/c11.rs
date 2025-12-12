use std::collections::{HashMap, HashSet};

use crate::{check_result2, utility};

const TEST: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

const TEST2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<String>,
}

type NodeMap = HashMap<String, Node>;

fn parse_line(l: &str) -> (String, Node) {
    let (name, sequence) = l.split_at(l.find(':').unwrap());
    let sequence = &sequence[2..];

    (
        name.to_string(),
        Node {
            edges: sequence.split(' ').map(|s| s.to_string()).collect(),
        },
    )
}

fn inverted_graph(map: &NodeMap) -> NodeMap {
    let mut new_map: NodeMap = HashMap::new();

    for (name, edges) in map {
        for edge in edges.edges.iter() {
            new_map
                .entry(edge.to_string())
                .and_modify(|e| e.edges.push(name.to_string()))
                .or_insert(Node {
                    edges: vec![name.to_string()],
                });
        }
    }

    new_map
}

fn count_paths(map: &NodeMap, current: &str, target: &str) -> usize {
    if current == target {
        return 1;
    }

    if let Some(node) = map.get(current) {
        let mut count = 0;
        for edge in node.edges.iter() {
            count += count_paths(map, &edge, target);
        }
        return count;
    }

    return 0;
}

fn count_paths_fft_dac(
    map: &NodeMap,
    current: &str,
    target: &str,
    mut reached: [bool; 2],
) -> usize {
    if current == target && reached.iter().all(|v| *v) {
        return 1;
    } else if current == target {
        return 0;
    }

    if current == "fft" {
        reached[0] = true;
    }

    if current == "dac" {
        reached[1] = true;
    }

    if let Some(node) = map.get(current) {
        let mut count = 0;
        for edge in node.edges.iter() {
            count += count_paths_fft_dac(map, &edge, target, reached);
        }
        return count;
    }

    return 0;
}

type MemoTable = HashMap<String, usize>;
fn count_paths_memo(map: &NodeMap, start: &str, target: &str) -> usize {
    fn count_paths_inner(
        memo: &mut MemoTable,
        map: &NodeMap,
        current: &str,
        target: &str,
    ) -> usize {
        if current == target {
            return 1;
        }

        if let Some(node) = map.get(current) {
            let mut count = 0;
            for edge in node.edges.iter() {
                let result = if let Some(&memo_value) = memo.get(edge) {
                    memo_value
                } else {
                    let value = count_paths_inner(memo, map, &edge, target);
                    memo.insert(edge.clone(), value);
                    value
                };

                count += result;
            }
            return count;
        }

        return 0;
    }

    let mut memo = MemoTable::new();
    let result = count_paths_inner(&mut memo, map, start, target);
    result
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 11).unwrap();

    let map: HashMap<String, Node> = input.lines().map(parse_line).collect();
    let inverted = inverted_graph(&map);

    //dbg!(&map);

    // let mut answer1 = 0;
    //let mut answer2 = 0;

    let answer1 = count_paths_memo(&map, "you", "out");

    let answer2 = {
        let svr_fft = count_paths_memo(&map, "svr", "fft");
        let fft_dac = count_paths_memo(&map, "fft", "dac");
        let dac_out = count_paths_memo(&map, "dac", "out");

        svr_fft * fft_dac * dac_out
    };

    (answer1, answer2)
}

check_result2!(552, 0);
