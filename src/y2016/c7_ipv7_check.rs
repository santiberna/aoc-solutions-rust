use crate::check_result;

struct IPV7 {
    pub sections: Vec<String>,
}

fn find_abba(str: &str) -> bool {
    str.as_bytes()
        .windows(4)
        .any(|s| s[0] == s[3] && s[1] == s[2] && s[0] != s[1])
}

fn find_all_abas(str: &str) -> Vec<[u8; 3]> {
    str.as_bytes()
        .windows(3)
        .filter(|s| s[0] == s[2] && s[0] != s[1])
        .map(|s| [s[0], s[1], s[2]])
        .collect()
}

impl IPV7 {
    fn new(str: &str) -> Self {
        let sections: Vec<String> = str
            .replace('[', " ")
            .replace(']', " ")
            .split(' ')
            .map(String::from)
            .collect();
        Self { sections }
    }

    fn has_tls_support(&self) -> bool {
        let mut found_tls = false;

        for (i, str) in self.sections.iter().enumerate() {
            let is_brackets = i % 2 == 1;
            let result = find_abba(str);

            if is_brackets && result {
                return false;
            } else if !is_brackets && result {
                found_tls = true;
            }
        }

        found_tls
    }

    fn has_ssl_support(&self) -> bool {
        let found_abas: Vec<[u8; 3]> = self
            .sections
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .flat_map(|(_, str)| find_all_abas(str))
            .collect();

        let found_matches: Vec<[u8; 3]> = self
            .sections
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .flat_map(|(_, str)| find_all_abas(str))
            .collect();

        for x in &found_abas {
            for y in &found_matches {
                if x[0] == y[1] && y[0] == x[1] {
                    return true;
                }
            }
        }

        false
    }
}

fn challenge(input: &str) -> (i64, i64) {
    let contents: Vec<IPV7> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(IPV7::new)
        .collect();

    let answer1: i64 = contents
        .iter()
        .map(IPV7::has_tls_support)
        .fold(0, |acc, v| if v { acc + 1 } else { acc });

    let answer2: i64 = contents
        .iter()
        .map(IPV7::has_ssl_support)
        .fold(0, |acc, v| if v { acc + 1 } else { acc });

    (answer1, answer2)
}

check_result!("input/Y2016/C7.txt", 118, 260);
