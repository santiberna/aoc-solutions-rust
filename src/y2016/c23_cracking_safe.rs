use crate::utility::assembunny;

fn run_seven(code: &Vec<assembunny::Instruction>) -> i64 {
    let mut vm = assembunny::VirtualMachine::default();
    *vm.get_register_mut('a') = 7;
    vm.run(&mut code.clone());
    *vm.get_register('a')
}

fn run_twelve(code: &Vec<assembunny::Instruction>) -> i64 {
    let mut vm = assembunny::VirtualMachine::default();
    *vm.get_register_mut('a') = 12;
    vm.run(&mut code.clone());
    *vm.get_register('a')
}

fn challenge(input: &str) -> (i64, i64) {
    let code: Vec<assembunny::Instruction> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| l.parse::<assembunny::Instruction>().unwrap())
        .collect();

    (run_seven(&code), run_twelve(&code))
}

//check_result!("input/Y2016/C23.txt", 0, 0);
