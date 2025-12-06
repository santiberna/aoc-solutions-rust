use crate::{
    check_result2,
    utility::{self, parsing},
};

const TEST: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

#[derive(Debug, Clone, Copy)]
enum Op {
    ADD,
    MUL,
}

fn parse_ops(line: &str) -> Vec<(Op, usize)> {
    line.char_indices()
        .filter_map(|(i, c)| match c {
            '*' => Some((Op::MUL, i)),
            '+' => Some((Op::ADD, i)),
            _ => None,
        })
        .collect()
}

fn challenge() -> (i64, i64) {
    let input = utility::input::get_input(2025, 6).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut answer1 = 0;
    let mut answer2 = 0;

    let operations = parse_ops(lines.last().unwrap());

    {
        let numbers = lines[0..lines.len() - 1]
            .iter()
            .map(|s| parsing::parse_all_numbers(*s))
            .collect::<Vec<_>>();

        for (i, op) in operations.iter().enumerate() {
            let numbers = numbers.iter().map(|ns| ns[i]).collect::<Vec<_>>();
            match op.0 {
                Op::ADD => answer1 += numbers.iter().sum::<i64>(),
                Op::MUL => answer1 += numbers.iter().product::<i64>(),
            };
        }
    }

    {
        let mut problem_sizes = vec![];

        let mut iter = operations.iter().peekable();
        while let Some((_, i)) = iter.next() {
            if let Some((_, n)) = iter.peek() {
                problem_sizes.push(*n - *i - 1);
            } else {
                problem_sizes.push(lines.last().unwrap().len() - *i);
            }
        }

        //dbg!(&operations);
        //dbg!(&problem_sizes);

        let number_lines = &lines[0..lines.len() - 1];

        let iter = operations
            .iter()
            .zip(problem_sizes.into_iter())
            .map(|a| (a.0.0, a.0.1, a.1));

        for (op, problem_start, problem_size) in iter {
            let mut numbers = vec![];
            for column in problem_start..problem_start + problem_size {
                let mut string = String::new();
                for line in number_lines {
                    let char = line.chars().nth(column).unwrap();
                    if char.is_digit(10) {
                        string.push(char);
                    }
                }
                let number = string.parse::<i64>().unwrap();
                numbers.push(number);
            }

            match op {
                Op::ADD => answer2 += numbers.iter().sum::<i64>(),
                Op::MUL => answer2 += numbers.iter().product::<i64>(),
            };
        }
    }

    (answer1, answer2)
}

check_result2!(3525371263915, 6846480843636);
