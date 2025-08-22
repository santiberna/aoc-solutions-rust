use std::collections::HashMap;

use crate::check_result;

fn challenge(input: &str) -> (String, String) {
    let contents: Vec<Vec<u8>> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|str| str.as_bytes().to_owned())
        .collect();

    let mut answer = Vec::new();
    let mut answer2 = Vec::new();

    for i in 0..contents[0].len() {
        let mut freq_map = HashMap::new();

        for j in 0..contents.len() {
            *freq_map.entry(contents[j][i]).or_insert(0) += 1usize;
        }

        let max_val_iter = freq_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
        let min_val_iter = freq_map.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().0;

        answer.push(*max_val_iter);
        answer2.push(*min_val_iter);
    }

    (
        String::from_utf8(answer).unwrap(),
        String::from_utf8(answer2).unwrap(),
    )
}

check_result!(
    "input/Y2016/C6.txt",
    "wkbvmikb".to_string(),
    "evakwaga".to_string()
);
