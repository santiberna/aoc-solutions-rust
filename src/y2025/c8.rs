use nalgebra::{Point3, Vector3};

use crate::{check_result2, utility};
type IVec3 = Vector3<i64>;

const TEST: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

fn parse_coord(s: &str) -> IVec3 {
    let mut iter = s.split(',').map(|s| s.parse().unwrap());
    IVec3::new(
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    )
}

fn distance(c1: &IVec3, c2: &IVec3) -> f32 {
    if c1 == c2 {
        return f32::INFINITY;
    }
    let p1 = Point3::new(c1.x as f32, c1.y as f32, c1.z as f32);
    let p2 = Point3::new(c2.x as f32, c2.y as f32, c2.z as f32);
    let distance = nalgebra::distance(&p1, &p2);
    distance
}

fn connections(
    coords: &Vec<IVec3>,
    distances: &Vec<(f32, usize, usize)>,
    count: usize,
) -> (usize, Vec<Option<usize>>) {
    let mut circuit_numbers: Vec<Option<usize>> = coords.iter().map(|_| None).collect();
    let mut current_circuit = 0;

    for d in &distances[0..count.min(distances.len())] {
        let n1 = circuit_numbers[d.1];
        let n2 = circuit_numbers[d.2];

        // println!("Connecting:");
        // let p1 = dbg!(coords[d.1]);
        // let p2 = dbg!(coords[d.2]);

        match (n1, n2) {
            (None, None) => {
                circuit_numbers[d.1] = Some(current_circuit);
                circuit_numbers[d.2] = Some(current_circuit);
                current_circuit += 1;
                //c += 1;
            }
            (Some(i), None) => {
                circuit_numbers[d.2] = Some(i);
                //c += 1;
            }
            (None, Some(i)) => {
                circuit_numbers[d.1] = Some(i);
                //c += 1
            }
            (Some(a), Some(b)) => {
                let iter = circuit_numbers.iter_mut().filter(|s| **s == Some(b));
                iter.for_each(|s| *s = Some(a));
            }
        }
    }

    (current_circuit, circuit_numbers)
}

fn last_connection(coords: &Vec<IVec3>, distances: &Vec<(f32, usize, usize)>) -> (IVec3, IVec3) {
    let mut circuit_numbers: Vec<Option<usize>> = coords.iter().map(|_| None).collect();
    let mut last: Option<(IVec3, IVec3)> = None;
    let mut current_circuit = 0;

    for d in distances {
        let n1 = circuit_numbers[d.1];
        let n2 = circuit_numbers[d.2];

        // println!("Connecting:");
        // let p1 = dbg!(coords[d.1]);
        // let p2 = dbg!(coords[d.2]);

        match (n1, n2) {
            (None, None) => {
                circuit_numbers[d.1] = Some(current_circuit);
                circuit_numbers[d.2] = Some(current_circuit);
                current_circuit += 1;
                //c += 1;
            }
            (Some(i), None) => {
                circuit_numbers[d.2] = Some(i);
                let p1 = coords[d.1];
                let p2 = coords[d.2];

                last = Some((p1, p2));
            }
            (None, Some(i)) => {
                circuit_numbers[d.1] = Some(i);
                let p1 = coords[d.1];
                let p2 = coords[d.2];

                last = Some((p1, p2));
            }
            (Some(a), Some(b)) => {
                if a != b {
                    let p1 = coords[d.1];
                    let p2 = coords[d.2];

                    last = Some((p1, p2));

                    let iter = circuit_numbers.iter_mut().filter(|s| **s == Some(b));
                    iter.for_each(|s| *s = Some(a));
                }
            }
        }
    }

    dbg!(last.unwrap())
}

fn challenge() -> (usize, usize) {
    let input = utility::input::get_input(2025, 8).unwrap();
    let coords = input.lines().map(parse_coord).collect::<Vec<_>>();

    println!("Problem size: {}", coords.len());

    let distances = {
        let mut distances: Vec<(f32, usize, usize)> = vec![];

        for n1 in 0..coords.len() - 1 {
            for n2 in n1 + 1..coords.len() {
                let c1 = coords[n1];
                let c2 = coords[n2];
                distances.push((distance(&c1, &c2), n1, n2));
            }
        }

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        distances
    };

    let answer1 = {
        //dbg!(&distances);

        let (circuit_count, circuit_numbers) = connections(&coords, &distances, 1000);

        let mut counts = vec![0; circuit_count];
        for v in &circuit_numbers {
            if let Some(v) = v {
                counts[*v] += 1
            }
        }

        counts.sort();
        counts[counts.len() - 3..].iter().product::<usize>()
    };

    let answer2 = {
        let (p1, p2) = last_connection(&coords, &distances);
        p1.x * p2.x
    };

    (answer1, answer2 as usize)
}

check_result2!(50760, 3206508875);
