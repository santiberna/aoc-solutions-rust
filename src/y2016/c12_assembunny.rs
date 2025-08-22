use crate::check_result;
use crate::utility::assembunny::{self, Instruction};

fn challenge(input: &str) -> (i64, i64) {

    let mut code: Vec<assembunny::Instruction> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.parse::<assembunny::Instruction>().unwrap())
        .collect();

    let mut vm = assembunny::VirtualMachine::default();
    vm.run(&mut code);

    let answer1 = *vm.get_register('a');

    let mut vm = assembunny::VirtualMachine::default();
    *vm.get_register_mut('c') = 1;
    vm.run(&mut code);

    let answer2 = *vm.get_register('a');

    (answer1, answer2)
}

check_result!("input/Y2016/C12.txt", 318003, 9227657);
