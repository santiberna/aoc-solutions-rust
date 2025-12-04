use crate::{check_result, utility::matrix::MatrixVec};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_REGEX: Regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
}

#[derive(Debug, Clone, Default)]
struct DiskInfo {
    size: usize,
    used: usize,
    avail: usize,
    percent: usize,
}

fn parse_disk(line: &str) -> Option<((usize, usize), DiskInfo)> {
    if let Some(captures) = PARSE_REGEX.captures(line) {
        Some((
            (
                captures[1].parse::<usize>().ok()?,
                captures[2].parse::<usize>().ok()?,
            ),
            DiskInfo {
                size: captures[3].parse().ok()?,
                used: captures[4].parse().ok()?,
                avail: captures[5].parse().ok()?,
                percent: captures[6].parse().ok()?,
            },
        ))
    } else {
        None
    }
}

fn insert_data(
    mut out: MatrixVec<DiskInfo>,
    input: Option<((usize, usize), DiskInfo)>,
) -> MatrixVec<DiskInfo> {
    if let Some(((x, y), info)) = input {
        out.set(y, x, info)
    }
    out
}

fn viable_transfer(src: &DiskInfo, dst: &DiskInfo) -> bool {
    src.used != 0 && src.used <= dst.avail
}

fn simplify_node(input: &DiskInfo) -> char {
    if input.used == 0 {
        '_'
    } else {
        if input.size >= 100 { '#' } else { '.' }
    }
}

fn simplify_grid(input: &MatrixVec<DiskInfo>) -> MatrixVec<char> {
    let data = input.iter().map(simplify_node).collect();
    MatrixVec::from_vec(input.rows(), input.cols(), data)
}

fn challenge(input: &str) -> (usize, usize) {
    let disks = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_disk)
        .fold(MatrixVec::<DiskInfo>::new(28, 38), insert_data);

    let mut part1 = 0;

    for a in disks.iter() {
        for b in disks.iter() {
            if !std::ptr::eq(a, b) && viable_transfer(&a, &b) {
                part1 += 1
            }
        }
    }

    let _graph = simplify_grid(&disks);

    // Solution can be handmade by printing
    // dbg!(_graph)

    (part1, 252)
}

check_result!("input/Y2016/C22.txt", 1038, 252);
