use crate::check_result;

fn challenge(_: &str) -> (i64, i64) {
    let mut test = 2;

    loop {
        //println!("{:b}", test);
        if test >= 2555 {
            //println!("{}", test - 2555);
            break;
        }

        test = test * 4 + 2;
    }

    (175, 0)
}

// fn generate_sequence(input: i64) {
//     let mut a = 0;
//     let mut d = input + 365 * 7;

//     // jump a == 0

//     a = d;

//     // jump a != 0

//     c = a % 2;
//     a = a / 2;
//     b = 2 - c;

//     println!("{} ", b) // out
// }

// k = 101010...

check_result!("input/Y2016/C25.txt", 175, 0);
