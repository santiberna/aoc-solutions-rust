use std::{
    collections::{BTreeSet, HashSet},
    io::Lines,
};

use itertools::Itertools;

use crate::{
    check_result2,
    utility::{self, directions::IVec2},
};

fn parse_vec2(elem: &str) -> IVec2 {
    let (x, y) = elem.split_at(elem.find(',').unwrap());
    let y = &y[1..];
    IVec2::new(x.parse().unwrap(), y.parse().unwrap())
}

const TEST: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

const TEST2: &str = r#"0,0
2,0
2,2
3,2
3,0
5,0
5,2
0,2"#;

// Separates edges into horizontal edges and vertical edges
fn polygon_lines(coords: &[IVec2]) -> (Vec<(IVec2, IVec2)>, Vec<(IVec2, IVec2)>) {
    let mut a = coords.chunks(2).map(|s| (s[0], s[1])).collect::<Vec<_>>();
    let mut b = {
        let mut ranges: Vec<(IVec2, IVec2)> = coords[1..coords.len() - 1]
            .chunks(2)
            .map(|s| (s[0], s[1]))
            .collect();
        ranges.push((*coords.first().unwrap(), *coords.last().unwrap()));
        ranges
    };

    let first = a.first().unwrap();

    if first.0.x == first.1.x {
        b.sort_by(|p1, p2| p1.0.y.cmp(&p2.0.y));
        a.sort_by(|p1, p2| p1.0.x.cmp(&p2.0.x));
        (b, a)
    } else {
        a.sort_by(|p1, p2| p1.0.y.cmp(&p2.0.y));
        b.sort_by(|p1, p2| p1.0.x.cmp(&p2.0.x));
        (a, b)
    }
}

fn generate_rects(h: &[(IVec2, IVec2)], v: &[(IVec2, IVec2)]) -> Vec<(IVec2, IVec2)> {
    let mut rects = vec![];

    // remove duplicates (horizontals on the same y value)
    let merged_horizontal = h.iter().map(|l| l.0.y).collect::<BTreeSet<i64>>();
    let merged_horizontal = merged_horizontal.into_iter().collect::<Vec<i64>>();

    fn make_range(a: i64, b: i64) -> (i64, i64) {
        if a > b { (b, a) } else { (a, b) }
    }

    for (parity, x_line) in merged_horizontal.windows(2).enumerate() {
        //let mut line = vec![];

        let start = x_line[0];
        let end = x_line[1];

        let x_values = v
            .iter()
            .filter(|(p1, p2)| {
                let range = make_range(p1.y, p2.y);
                start >= range.0 && end <= range.1
            })
            .map(|p| p.0.x)
            .collect::<BTreeSet<i64>>();

        let x_values = x_values.into_iter().collect::<Vec<_>>();

        assert!(x_values.len() % 2 == 0);

        for xs in x_values.chunks(2) {
            let x1 = xs[0];
            let x2 = xs[1];

            let y1 = start;
            let y2 = end;

            if parity % 2 == 1 {}

            let rect = (IVec2::new(x1, y1), IVec2::new(x2, y2));
            rects.push(rect);
        }

        //rects.push(line);
    }
    rects
}

// This is not even necessary with correct area intersection check
fn check_corners((a, b): &(IVec2, IVec2), rects: &[(IVec2, IVec2)]) -> bool {
    let a = *a;
    let b = *b;
    let c = IVec2::new(a.x, b.y);
    let d = IVec2::new(b.x, a.y);

    fn inside_rect(p: &IVec2, a: &IVec2, b: &IVec2) -> bool {
        p.x >= a.x && p.x <= b.x && p.y >= a.y && p.y <= b.y
    }

    fn inside_rects(p: &IVec2, rects: &[(IVec2, IVec2)]) -> bool {
        for (a, b) in rects {
            if inside_rect(p, a, b) {
                return true;
            }
        }
        false
    }

    let corners = [a, b, c, d];
    corners.iter().all(|p| inside_rects(p, rects))
}

fn calc_area_inclusive(rect: &(IVec2, IVec2)) -> i64 {
    let x = (rect.1.x - rect.0.x) + 1;
    let y = (rect.1.y - rect.0.y) + 1;
    x * y
}

fn calc_area(rect: &(IVec2, IVec2)) -> i64 {
    let x = rect.1.x - rect.0.x;
    let y = rect.1.y - rect.0.y;
    x * y
}

fn check_small_lines_dont_exist_assumptions(ns: &[IVec2]) {
    let lines = ns.windows(2).map(|s| (s[0], s[1])).collect::<Vec<_>>();
    let small_lines = lines
        .iter()
        .filter(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs() < 1)
        .collect::<Vec<_>>();

    assert!(small_lines.is_empty())
}

fn intersect(a: &(IVec2, IVec2), b: &(IVec2, IVec2)) -> Option<(IVec2, IVec2)> {
    let min = IVec2::new(a.0.x.max(b.0.x), a.0.y.max(b.0.y));
    let max = IVec2::new(a.1.x.min(b.1.x), a.1.y.min(b.1.y));

    if min.x < max.x && min.y < max.y {
        Some((min, max))
    } else {
        None
    }
}

fn check_areas(rect: &(IVec2, IVec2), rects: &[(IVec2, IVec2)]) -> bool {
    rects.iter().fold(0, |acc, r| {
        acc + intersect(rect, r).map(|r| calc_area(&r)).unwrap_or(0)
    }) == calc_area(rect)
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 9).unwrap();
    let numbers = input.lines().map(parse_vec2).collect::<Vec<_>>();

    let answer1 = {
        let mut max = 0;
        for vec in numbers.iter().combinations(2) {
            let (n, m) = (vec[0], vec[1]);
            let rect = (
                IVec2::new(n.x.min(m.x), n.y.min(m.y)),
                IVec2::new(n.x.max(m.x), n.y.max(m.y)),
            );
            max = calc_area_inclusive(&rect).max(max);
        }
        max
    };

    let answer2 = {
        let mut max = 0;
        check_small_lines_dont_exist_assumptions(&numbers);

        let (h, v) = polygon_lines(&numbers);
        let rects = generate_rects(&h, &v);

        for vec in numbers.iter().combinations(2) {
            let (n, m) = (vec[0], vec[1]);
            let rect = (
                IVec2::new(n.x.min(m.x), n.y.min(m.y)),
                IVec2::new(n.x.max(m.x), n.y.max(m.y)),
            );

            if
            /*check_corners(&rect, &rects) &&*/
            check_areas(&rect, &rects) {
                max = calc_area_inclusive(&rect).max(max);
            }
        }

        max
    };

    (answer1, answer2)
}

check_result2!(4764078684, 1652344888);
