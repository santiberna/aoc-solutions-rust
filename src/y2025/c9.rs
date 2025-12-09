use std::{
    collections::{BTreeSet, HashSet},
    io::Lines,
};

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
4,2
4,0
5,0
5,2
0,2"#;

// (4, 4)

fn within_range_exclusive(v: i64, a: i64, b: i64) -> bool {
    (v > a && v < b) || (v < a && v > b)
}

fn within_range_inclusive(v: i64, a: i64, b: i64) -> bool {
    (v >= a && v <= b) || (v <= a && v >= b)
}

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

fn make_range(a: i64, b: i64) -> (i64, i64) {
    if a > b { (b, a) } else { (a, b) }
}

fn inside_rect(p: &IVec2, a: &IVec2, b: &IVec2) -> bool {
    p.x >= a.x && p.x <= b.x && p.y >= a.y && p.y <= b.y
}

fn inside_rects(p: &IVec2, rects: &[(IVec2, IVec2)]) -> bool {
    for (a, b) in rects {
        if inside_rect(p, a, b) {
            return true;
        }
    }
    return false;
}

fn rect_no_edge_out(rect: &(IVec2, IVec2), h: &[(IVec2, IVec2)], v: &[(IVec2, IVec2)]) -> bool {
    let ha = (
        IVec2::new(rect.0.x, rect.0.y),
        IVec2::new(rect.1.x, rect.0.y),
    );
    let hb = (
        IVec2::new(rect.0.x, rect.1.y),
        IVec2::new(rect.1.x, rect.1.y),
    );

    // dbg!(&ha);
    // dbg!(&hb);

    for v_intersect in v.iter() {
        let vert_x = v_intersect.0.x;

        //dbg!(v_intersect);
        //dbg!(vert_x);
        //dbg!(&ha);

        if within_range_exclusive(vert_x, ha.0.x, ha.1.x)
            && within_range_inclusive(ha.0.y, v_intersect.0.y, v_intersect.1.y)
        {
            let y = ha.0.y;
            let (y1, y2) = if v_intersect.0.y < v_intersect.1.y {
                (v_intersect.0.y, v_intersect.1.y)
            } else {
                (v_intersect.1.y, v_intersect.0.y)
            };

            if !(y == y2 && y >= y1) {
                return false;
            }
        }

        //dbg!(&hb);

        if within_range_exclusive(vert_x, hb.0.x, hb.1.x)
            && within_range_inclusive(hb.0.y, v_intersect.0.y, v_intersect.1.y)
        {
            let y = hb.0.y;
            let (y1, y2) = if v_intersect.0.y < v_intersect.1.y {
                (v_intersect.0.y, v_intersect.1.y)
            } else {
                (v_intersect.1.y, v_intersect.0.y)
            };

            if !(y == y2 && y <= y1) {
                return false;
            }
        }
    }

    let va = (
        IVec2::new(rect.0.x, rect.0.y),
        IVec2::new(rect.0.x, rect.1.y),
    );
    let vb = (
        IVec2::new(rect.1.x, rect.0.y),
        IVec2::new(rect.1.x, rect.1.y),
    );

    // dbg!(&va);
    // dbg!(&vb);

    for h_intersect in h.iter() {
        let hori_y = h_intersect.0.y;

        if within_range_exclusive(hori_y, va.0.y, va.1.y)
            && within_range_inclusive(va.0.x, h_intersect.0.x, h_intersect.1.y)
        {
            let x = va.0.x;
            let (x1, x2) = if h_intersect.0.x < h_intersect.1.x {
                (h_intersect.0.x, h_intersect.1.x)
            } else {
                (h_intersect.1.x, h_intersect.0.x)
            };

            if !(x == x2 && x >= x1) {
                return false;
            }
        }

        if within_range_exclusive(hori_y, va.0.y, va.1.y)
            && within_range_inclusive(vb.0.x, h_intersect.0.x, h_intersect.1.y)
        {
            let x = va.0.x;
            let (x1, x2) = if h_intersect.0.x < h_intersect.1.x {
                (h_intersect.0.x, h_intersect.1.x)
            } else {
                (h_intersect.1.x, h_intersect.0.x)
            };

            if !(x == x2 && x <= x1) {
                return false;
            }
        }
    }

    true
}

fn generate_rects(h: &[(IVec2, IVec2)], v: &[(IVec2, IVec2)]) -> Vec<(IVec2, IVec2)> {
    let mut rects = vec![];

    let merged_horizontal = h.iter().map(|l| l.0.y).collect::<BTreeSet<i64>>();
    let merged_horizontal = merged_horizontal.into_iter().collect::<Vec<i64>>();

    for x_line in merged_horizontal.windows(2) {
        let start = x_line[0];
        let end = x_line[1];

        let x_values = v
            .iter()
            .filter(|(p1, p2)| {
                let range = make_range(p1.y, p2.y);
                start >= range.0 && end <= range.1
            })
            .map(|p| p.0.x)
            .collect::<Vec<_>>();

        assert!(x_values.len() % 2 == 0);

        for xs in x_values.chunks(2) {
            let x1 = xs[0];
            let x2 = xs[1];

            let y1 = start;
            let y2 = end;

            let rect = (IVec2::new(x1, y1), IVec2::new(x2, y2));
            rects.push(rect);
        }
    }
    rects
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 9).unwrap();
    let numbers = input.lines().map(parse_vec2).collect::<Vec<_>>();

    let answer1 = {
        let mut max = 0;
        for n in &numbers {
            for m in &numbers {
                let x = (n.x - m.x).abs() + 1;
                let y = (n.y - m.y).abs() + 1;
                max = (x * y).max(max);
            }
        }
        max
    };

    let answer2 = {
        let mut max = 0;
        let (h, v) = polygon_lines(&numbers);
        let rects = generate_rects(&h, &v);

        let valid = rects.iter().all(|(p1, p2)| p1.x <= p2.x && p1.y <= p2.y);
        assert!(valid);

        assert!(rect_no_edge_out(
            &(IVec2::new(2, 3), IVec2::new(9, 5)),
            &h,
            &v
        ));

        for n in &numbers {
            for m in &numbers {
                let a = *n;
                let b = *m;
                let c = IVec2::new(a.x, b.y);
                let d = IVec2::new(b.x, a.y);

                let corners = [a, b, c, d];

                if corners.iter().all(|p| inside_rects(p, &rects)) {
                    let rect = (
                        IVec2::new(a.x.min(b.x), a.y.min(b.y)),
                        IVec2::new(a.x.max(b.x), a.y.max(b.y)),
                    );

                    let x = (n.x - m.x).abs() + 1;
                    let y = (n.y - m.y).abs() + 1;
                    let area = x * y;

                    //dbg!(&rect);
                    //dbg!(area);

                    if rect_no_edge_out(&rect, &h, &v) {
                        println!("PASSED!");
                        max = area.max(max);
                    }
                }
            }
        }

        max
    };

    (answer1, answer2)
}

check_result2!(0, 0);
