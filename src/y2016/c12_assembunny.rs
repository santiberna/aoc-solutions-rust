use crate::check_result;

type Literal = i64;
type Register = char;

#[derive(Debug)]
enum Parameter {
    Value(Literal),
    Address(Register),
}

#[derive(Debug)]
enum Instruction {
    CPY(Parameter, Register),
    INC(Register),
    DEC(Register),
    JNZ(Parameter, Literal),
}

const REGISTER_COUNT: usize = (b'z' - b'a') as usize;

#[derive(Default)]
struct VirtualMachine {
    instruction_ptr: usize,
    registers: [Literal; REGISTER_COUNT],
}

impl VirtualMachine {
    fn run(&mut self, code: &Vec<Instruction>) {
        self.instruction_ptr = 0;

        while self.instruction_ptr < code.len() {
            let instruction = &code[self.instruction_ptr];

            match instruction {
                Instruction::CPY(from, to) => {
                    match from {
                        Parameter::Address(a) => {
                            *self.get_register_mut(*to) = *self.get_register(*a)
                        }
                        Parameter::Value(v) => *self.get_register_mut(*to) = *v,
                    }
                    self.instruction_ptr += 1;
                }
                Instruction::INC(a) => {
                    *self.get_register_mut(*a) += 1;
                    self.instruction_ptr += 1;
                }
                Instruction::DEC(a) => {
                    *self.get_register_mut(*a) -= 1;
                    self.instruction_ptr += 1;
                }
                Instruction::JNZ(p, l) => {
                    let v = match p {
                        Parameter::Address(a) => *self.get_register(*a),
                        Parameter::Value(v) => *v,
                    };

                    if (v) != 0 {
                        self.instruction_ptr = (self.instruction_ptr as i64 + (*l)) as usize;
                    } else {
                        self.instruction_ptr += 1;
                    }
                }
            }
        }
    }

    fn get_register(&self, register: char) -> &Literal {
        let index = register as usize - 'a' as usize;
        &self.registers[index]
    }

    fn get_register_mut(&mut self, register: char) -> &mut Literal {
        let index = register as usize - 'a' as usize;
        &mut self.registers[index]
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let words: Vec<&str> = s.split(' ').collect();

    match words[0] {
        "cpy" => {
            if let Ok(literal) = words[1].parse::<i64>() {
                Instruction::CPY(Parameter::Value(literal), words[2].chars().nth(0).unwrap())
            } else {
                Instruction::CPY(
                    Parameter::Address(words[1].chars().nth(0).unwrap()),
                    words[2].chars().nth(0).unwrap(),
                )
            }
        }
        "inc" => Instruction::INC(words[1].chars().nth(0).unwrap()),
        "dec" => Instruction::DEC(words[1].chars().nth(0).unwrap()),
        "jnz" => {
            if let Ok(literal) = words[1].parse::<i64>() {
                Instruction::JNZ(Parameter::Value(literal), words[2].parse().unwrap())
            } else {
                Instruction::JNZ(
                    Parameter::Address(words[1].chars().nth(0).unwrap()),
                    words[2].parse().unwrap(),
                )
            }
        }
        _ => unreachable!(),
    }
}

fn challenge(input: &str) -> (i64, i64) {
    let code: Vec<Instruction> = std::fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(parse_instruction)
        .collect();

    let mut vm = VirtualMachine::default();
    vm.run(&code);

    let answer1 = *vm.get_register('a');

    let mut vm = VirtualMachine::default();
    *vm.get_register_mut('c') = 1;
    vm.run(&code);

    let answer2 = *vm.get_register('a');

    (answer1, answer2)
}

check_result!("input/C12.txt", 318003, 9227657);
