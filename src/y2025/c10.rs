use crate::{
    check_result2,
    utility::{self, matrix::MatrixVec},
};
use num::Rational64;

const TEST: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

fn parse_csv(s: &str) -> Vec<i64> {
    s.split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_line(s: &str) -> (Vec<bool>, Vec<Vec<i64>>, Vec<i64>) {
    let splits = s.trim().split(' ').collect::<Vec<_>>();

    let indicators = {
        let indicators = *splits.first().unwrap();
        let indicators = &indicators[1..indicators.len() - 1];
        indicators
            .bytes()
            .map(|c| match c {
                b'.' => false,
                b'#' => true,
                _ => panic!(),
            })
            .collect::<Vec<_>>()
    };
    let buttons = {
        let mut buttons = vec![];
        for s in &splits[1..splits.len() - 1] {
            let s = &s[1..s.len() - 1];
            buttons.push(parse_csv(s));
        }
        buttons
    };
    let voltages = {
        let v = *splits.last().unwrap();
        let v = &v[1..v.len() - 1];
        parse_csv(v)
    };
    (indicators, buttons, voltages)
}

#[derive(Debug)]
struct Solution<T> {
    zero_solution: Vec<T>,
    basis_vectors: Vec<Vec<T>>,
}

fn button_to_coefficients(target_size: usize, button: &[i64]) -> Vec<i64> {
    let mut out = vec![0; target_size];
    for p in button {
        out[*p as usize] = 1;
    }
    out
}

fn extract_pivot_cols(aug: &MatrixVec<i64>) -> Vec<Option<usize>> {
    let mut out = Vec::with_capacity(aug.rows());

    for i in 0..aug.rows() {
        let row = aug.get_row(i).unwrap();
        let row = &row[..row.len() - 1];
        let pivot_col = row.iter().position(|&v| v != 0);
        out.push(pivot_col);
    }

    out
}

fn make_augmented_matrix(target: &[i64], buttons: &[Vec<i64>]) -> MatrixVec<i64> {
    let button_coefficients = buttons
        .iter()
        .map(|v| button_to_coefficients(target.len(), v))
        .collect::<Vec<_>>();

    let mut a = MatrixVec::<i64>::new(target.len(), buttons.len() + 1);

    for i in 0..target.len() {
        let coeficients = button_coefficients.iter().map(|v| v[i]).collect::<Vec<_>>();
        for c in 0..coeficients.len() {
            *a.get_mut(i, c).unwrap() = coeficients[c];
        }
    }
    for i in 0..target.len() {
        *a.get_mut(i, buttons.len()).unwrap() = target[i];
    }
    a
}

////////////////////////
/// Mod2 arithmetic
///

fn forward_elimination_mod2(mut aug: MatrixVec<i64>) -> MatrixVec<i64> {
    let m = aug.rows();
    let n = aug.cols() - 1;

    let mut free_columns = vec![];
    let mut current_row = 0;

    for col in 0..n {
        let pivot_row = aug.col_iter(col).skip(current_row).position(|v| *v != 0);

        let Some(pivot_row) = pivot_row else {
            free_columns.push(col);
            continue;
        };

        let pivot_row = pivot_row + current_row;
        aug.swap_rows(pivot_row, current_row);
        let pivot_row = aug.get_row(current_row).unwrap().to_vec();

        for r in current_row + 1..m {
            let target_val = *aug.get(r, col).unwrap();

            if target_val != 0 {
                let row_target = aug.get_row_mut(r).unwrap();
                row_target
                    .iter_mut()
                    .zip(pivot_row.iter())
                    .for_each(|(v, i)| *v = (*v + *i) % 2);
            }
        }

        current_row += 1;

        if current_row == m {
            break;
        }
    }
    aug
}

fn generate_solution_mod2(aug: &MatrixVec<i64>, mut vars: Vec<i64>) -> Vec<i64> {
    let m = aug.rows();
    let n = aug.cols() - 1;
    let pivot_cols = extract_pivot_cols(aug);

    assert!(vars.len() == n);

    for row in (0..m).rev() {
        let found_col = pivot_cols[row];

        if let Some(pivot_col) = found_col {
            let mut last_val = *aug.get(row, n).unwrap(); // last column val

            for col in (pivot_col + 1)..n {
                let v = *aug.get(row, col).unwrap();
                last_val = (last_val + v * vars[col]) % 2
            }

            // no division needed for last step as pivot value is guaranteed to be 1
            vars[pivot_col] = last_val;
        }
    }

    vars
}

fn back_substituition_mod2(aug: &MatrixVec<i64>) -> Option<Solution<i64>> {
    for row in aug.iter_rows() {
        let last = row[row.len() - 1];
        let all_zeros = row[0..row.len() - 1].iter().all(|v| *v == 0);
        if all_zeros && last != 0 {
            return None;
        }
    }

    let n = aug.cols() - 1;

    let pivot_cols = extract_pivot_cols(aug);
    let free_variables = (0..n)
        .filter(|i| !pivot_cols.contains(&Some(*i)))
        .collect::<Vec<_>>();

    // Generate zero solution
    let zero_solution = vec![0; n];
    let zero_solution = generate_solution_mod2(aug, zero_solution);

    // Generate basis vectors
    let mut basis_vectors = vec![];
    for free_var in free_variables {
        let mut input = vec![0; n];
        input[free_var] = 1;
        basis_vectors.push(generate_solution_mod2(aug, input));
    }

    basis_vectors.iter_mut().for_each(|v| {
        v.iter_mut()
            .zip(zero_solution.iter())
            .for_each(|(v, i)| *v ^= *i);
    });

    Some(Solution {
        zero_solution,
        basis_vectors,
    })
}

////////////////////////
/// Regular arithmetic
///

fn forward_elimination(mut aug: MatrixVec<i64>) -> MatrixVec<i64> {
    let m = aug.rows();
    let n = aug.cols() - 1;

    let mut free_columns = vec![];
    let mut current_row = 0;
    let mut prev_pivot = 1;

    for col in 0..n {
        let pivot_row = aug.col_iter(col).skip(current_row).position(|v| *v != 0);

        let Some(pivot_row) = pivot_row else {
            free_columns.push(col);
            continue;
        };

        let pivot_row = pivot_row + current_row;
        aug.swap_rows(pivot_row, current_row);

        let pivot_row = aug.get_row(current_row).unwrap().to_vec();
        let pivot_val = *aug.get(current_row, col).unwrap();

        for r in current_row + 1..m {
            let target_val = *aug.get(r, col).unwrap();

            if target_val != 0 {
                let row_target = aug.get_row_mut(r).unwrap();
                row_target
                    .iter_mut()
                    .zip(pivot_row.iter())
                    .for_each(|(v, i)| *v = (*v * pivot_val) - (*i * target_val));
            }
        }

        prev_pivot = pivot_val;
        current_row += 1;

        if current_row == m {
            break;
        }
    }
    aug
}

fn generate_solution(aug: &MatrixVec<i64>, vars: Vec<i64>) -> Vec<Rational64> {
    let m = aug.rows();
    let n = aug.cols() - 1;
    let pivot_cols = extract_pivot_cols(aug);

    let mut vars = vars
        .into_iter()
        .map(Rational64::from_integer)
        .collect::<Vec<_>>();

    //dbg!(&pivot_cols);
    assert!(vars.len() == n);

    for row in (0..m).rev() {
        let found_col = pivot_cols[row];

        if let Some(pivot_col) = found_col {
            let pivot_val = Rational64::from_integer(*aug.get(row, pivot_col).unwrap());
            let mut last_val = Rational64::from_integer(*aug.get(row, n).unwrap()); // last column val

            for col in (pivot_col + 1)..n {
                let v = Rational64::from_integer(*aug.get(row, col).unwrap());
                last_val -= v * vars[col]
            }

            let v = last_val / pivot_val;
            vars[pivot_col] = v;
        }
    }

    vars
}

fn back_substituition(aug: &MatrixVec<i64>) -> Option<Solution<Rational64>> {
    for row in aug.iter_rows() {
        let last = row[row.len() - 1];
        let all_zeros = row[0..row.len() - 1].iter().all(|v| *v == 0);
        if all_zeros && last != 0 {
            return None;
        }
    }

    let n = aug.cols() - 1;

    let pivot_cols = extract_pivot_cols(aug);
    let free_variables = (0..n)
        .filter(|i| !pivot_cols.contains(&Some(*i)))
        .collect::<Vec<_>>();

    // Generate zero solution
    let zero_solution = vec![0; n];
    let zero_solution = generate_solution(aug, zero_solution);

    // Generate basis vectors
    let mut basis_vectors = vec![];
    for free_var in free_variables {
        let mut input = vec![0; n];
        input[free_var] = 1;
        basis_vectors.push(generate_solution(aug, input));
    }

    basis_vectors.iter_mut().for_each(|v| {
        v.iter_mut()
            .zip(zero_solution.iter())
            .for_each(|(v, i)| *v -= *i);
    });

    Some(Solution {
        zero_solution,
        basis_vectors,
    })
}

fn sum_mul_basis(mut vec: Vec<Rational64>, basis: &[Rational64], k: i64) -> Vec<Rational64> {
    assert!(vec.len() == basis.len());
    vec.iter_mut().zip(basis).for_each(|(v, i)| *v += i * k);
    vec
}

fn xor(mut vec: Vec<i64>, basis: &[i64], k: i64) -> Vec<i64> {
    assert!(vec.len() == basis.len());
    vec.iter_mut()
        .zip(basis)
        .for_each(|(v, i)| *v = (*v + i * k) % 2);
    vec
}

fn search_min_solution_mod2(current: &[i64], basis: &[Vec<i64>]) -> i64 {
    let mut out = i64::MAX;
    for k in 0..2 {
        let current = xor(current.to_vec(), &basis[0], k as i64);
        if basis.len() == 1 {
            out = out.min(current.iter().sum::<i64>());
        } else {
            out = out.min(search_min_solution_mod2(&current, &basis[1..]))
        }
    }
    out
}

fn search_min_solution(current: &[Rational64], basis: &[Vec<Rational64>], check: usize) -> i64 {
    let mut out = i64::MAX;
    for k in 0..check {
        let current = sum_mul_basis(current.to_vec(), &basis[0], k as i64);
        if basis.len() == 1 {
            out = out.min(valid_sequence(&current).unwrap_or(i64::MAX));
        } else {
            out = out.min(search_min_solution(&current, &basis[1..], check))
        }
    }
    out
}

fn assert_solution(sol: &[Rational64], aug: &MatrixVec<i64>) -> bool {
    let m = aug.rows();
    let n = aug.cols() - 1;
    assert!(sol.len() == n);

    for i in 0..m {
        let mut sum = Rational64::default();
        for j in 0..n {
            let v = Rational64::from_integer(*aug.get(i, j).unwrap());
            sum += v * sol[j];
        }

        let sum = sum.reduced();
        assert!(*sum.denom() == 1);

        if *sum.numer() != *aug.get(i, n).unwrap() {
            return false;
        }
    }
    true
}

fn valid_sequence(sequence: &[Rational64]) -> Option<i64> {
    if sequence
        .iter()
        .map(|v| v.reduced())
        .all(|v| *v.denom() == 1 && *v.numer() >= 0)
    {
        let result = sequence
            .iter()
            .fold(Rational64::default(), |acc, v| acc + v);

        Some(*result.reduced().numer())
    } else {
        None
    }
}

pub fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 10).unwrap();
    let data = input.lines().map(parse_line).collect::<Vec<_>>();
    let mut answer1 = 0;
    let mut answer2 = 0;

    {
        for (target, buttons, _) in data.iter() {
            let b = target
                .iter()
                .map(|t| if *t { 1 } else { 0 })
                .collect::<Vec<i64>>();

            let aug = make_augmented_matrix(&b, buttons);
            let aug = forward_elimination_mod2(aug.clone());
            let sol = back_substituition_mod2(&aug).unwrap();

            let sum = if sol.basis_vectors.is_empty() {
                sol.zero_solution.iter().sum::<i64>()
            } else {
                search_min_solution_mod2(&sol.zero_solution, &sol.basis_vectors)
            };

            answer1 += sum;
        }
    }

    {
        for (_, buttons, voltages) in data.iter() {
            let aug = make_augmented_matrix(voltages, buttons);
            let aug = forward_elimination(aug);
            let sol = back_substituition(&aug).unwrap();

            assert!(assert_solution(&sol.zero_solution, &aug));

            let sum = if sol.basis_vectors.is_empty() {
                valid_sequence(&sol.zero_solution).unwrap()
            } else {
                search_min_solution(&sol.zero_solution, &sol.basis_vectors, 150)
            };

            assert!(sum != i64::MAX);
            answer2 += sum;
        }
    }

    (answer1, answer2)
}

check_result2!(452, 17424);
