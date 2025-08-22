use crate::check_result;

struct Marker {
    pub length: usize,
    pub repeat: usize,
}

impl std::str::FromStr for Marker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once('x')
            .ok_or("Marker should contain a x in the middle")?;

        Ok(Marker {
            length: l.parse().map_err(|_| "Not a number")?,
            repeat: r.parse().map_err(|_| "Not a number")?,
        })
    }
}

fn decompress_count(input: &str) -> (usize, usize) {
    let mut counter = 0;
    let mut rec_counter = 0;

    let mut iter = input.chars();

    while let Some(c) = iter.next() {
        if c == '(' {
            let marker = iter
                .by_ref()
                .take_while(|&c| c != ')')
                .collect::<String>()
                .parse::<Marker>()
                .unwrap();

            let data: String = iter.by_ref().take(marker.length).collect();
            counter += data.len() * marker.repeat;

            let recursive_count = decompress_count(&data).1;
            rec_counter += recursive_count * marker.repeat;
        } else {
            counter += 1;
            rec_counter += 1;
        }
    }

    (counter, rec_counter)
}

fn challenge(input: &str) -> (usize, usize) {
    let contents: String = std::fs::read_to_string(input).unwrap();

    decompress_count(&contents)
}

check_result!("input/Y2016/C9.txt", 152851, 11797310782);
