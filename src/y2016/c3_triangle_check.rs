use crate::check_result;

type Triangle = [i64; 3];

fn parse_line(str: &str) -> Triangle {
    str.split(' ')
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<i64>>()
        .try_into()
        .unwrap()
}

fn transform_vertical(triangles: &Vec<Triangle>) -> Vec<Triangle> {
    assert!(triangles.len() % 3 == 0);

    let mut out = Vec::new();

    for j in 0..3usize {
        for i in 0..(triangles.len() / 3) {
            let collumn = j;
            let row = i * 3;

            let a = triangles[row][collumn];
            let b = triangles[row + 1][collumn];
            let c = triangles[row + 2][collumn];

            out.push([a, b, c]);
        }
    }

    out
}

fn check_triangle(triangle: &Triangle) -> bool {
    let mut t = triangle.clone();
    t.sort();

    assert!(t[2] >= t[1] && t[1] >= t[0]);
    t[0] + t[1] > t[2]
}

fn challenge(input: &str) -> (i64, i64) {
    let horizontal: Vec<[i64; 3]> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();

    let vertical = transform_vertical(&horizontal);

    let fold_op = |acc, &v| {
        if check_triangle(&v) { acc + 1 } else { acc }
    };

    (
        horizontal.iter().fold(0i64, fold_op),
        vertical.iter().fold(0i64, fold_op),
    )
}

check_result!("input/C3.txt", 862, 1577);
