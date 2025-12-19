use std::collections::HashMap;

use crate::{check_result2, utility::directions};

fn spiral_value(x: i64, y: i64) -> i64 {
    let k = x.abs().max(y.abs());
    let m = (2 * k + 1).pow(2);

    if y == -k {
        m - (k - x)
    } else if x == -k {
        m - 2 * k - (k + y)
    } else if y == k {
        m - 4 * k - (k + x)
    } else {
        // x == k
        m - 6 * k - (k - y)
    }
}

fn spiral_coords(n: i64) -> (i64, i64) {
    if n == 1 {
        return (0, 0);
    }

    let k = (((n as f64).sqrt() - 1.0) / 2.0).ceil() as i64;

    let m = (2 * k + 1).pow(2);
    let t = 2 * k;
    let d = m - n;

    if d < t {
        (k - d, -k)
    } else if d < 2 * t {
        (-k, -k + (d - t))
    } else if d < 3 * t {
        (-k + (d - 2 * t), k)
    } else {
        (k, k - (d - 3 * t))
    }
}

#[derive(Clone, Copy, Debug)]
struct Spiral {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    steps_left: i64,
    step_size: i64,
    turns: i64,
    current: i64,
}

impl Spiral {
    fn new() -> Self {
        Spiral {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
            steps_left: 1,
            step_size: 1,
            turns: 0,
            current: 1,
        }
    }
}

impl Iterator for Spiral {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = (self.x, self.y);

        self.x += self.dx;
        self.y += self.dy;
        self.steps_left -= 1;

        if self.steps_left == 0 {
            let (new_dx, new_dy) = (-self.dy, self.dx);
            self.dx = new_dx;
            self.dy = new_dy;

            self.turns += 1;
            if self.turns % 2 == 0 {
                self.step_size += 1;
            }
            self.steps_left = self.step_size;
        }

        self.current += 1;
        Some(pos)
    }
}

fn find_part2(input: i64) -> i64 {
    let mut map = HashMap::<(i64, i64), i64>::new();
    map.insert((0, 0), 1);

    for (x, y) in Spiral::new().skip(1) {
        let mut sum = 0;

        for dir in directions::ALL {
            sum += map.get(&(x + dir.x, y + dir.y)).unwrap_or(&0);
        }

        map.insert((x, y), sum);

        if sum > input {
            return sum;
        }
    }

    unreachable!()
}

fn challenge() -> (i64, i64) {
    let input = 368078;
    let coords = spiral_coords(input);

    (coords.0.abs() + coords.1.abs(), find_part2(input))
}

check_result2!(371, 1244);
