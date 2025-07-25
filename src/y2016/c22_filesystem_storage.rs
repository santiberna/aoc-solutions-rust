use std::collections::HashMap;

use crate::check_result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PARSE_REGEX: Regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();
}

#[derive(Debug)]
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

type Coordinates = (usize, usize);

fn challenge(input: &str) -> (usize, usize) {
    let disks: HashMap<Coordinates, DiskInfo> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_disk)
        .map(|o| o.unwrap())
        .collect();

    let mut part1 = 0;

    for (key1, a) in &disks {
        for (key2, b) in &disks {
            if *key1 == *key2 {
                continue;
            }

            if a.used != 0 && a.used <= b.avail {
                part1 += 1
            }
        }
    }

    (part1, 0)
}

check_result!("input/C22.txt", 0, 0);
