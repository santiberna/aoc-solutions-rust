type Literal = i64;
type Register = char;

#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    Value(Literal),
    Address(Register),
}

impl std::str::FromStr for Parameter {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(literal) = s.parse::<i64>() {
            Ok(Parameter::Value(literal))
        } else {
            Ok(Parameter::Address(s.chars().nth(0).ok_or("Parse Error")?))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    CPY(Parameter, Parameter),
    INC(Parameter),
    DEC(Parameter),
    JNZ(Parameter, Parameter),
    TGL(Parameter),
    OUT(Parameter),
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();

        match words[0] {
            "cpy" => Ok(Instruction::CPY(words[1].parse()?, words[2].parse()?)),
            "jnz" => Ok(Instruction::JNZ(words[1].parse()?, words[2].parse()?)),
            "inc" => Ok(Instruction::INC(words[1].parse()?)),
            "dec" => Ok(Instruction::DEC(words[1].parse()?)),
            "tgl" => Ok(Instruction::TGL(words[1].parse()?)),
            "out" => Ok(Instruction::TGL(words[1].parse()?)),
            _ => Err("Unknown Instruction"),
        }
    }
}

const REGISTER_COUNT: usize = (b'z' - b'a') as usize;

#[derive(Default)]
pub struct VirtualMachine {
    instruction_ptr: usize,
    registers: [Literal; REGISTER_COUNT],
}

impl VirtualMachine {
    pub fn run(&mut self, code: &mut Vec<Instruction>) -> Option<i64> {
        self.instruction_ptr = 0;

        while self.instruction_ptr < code.len() {
            let instruction = code[self.instruction_ptr];

            match instruction {
                Instruction::CPY(from, to) => {
                    if let Parameter::Address(t) = to {
                        match from {
                            Parameter::Address(a) => {
                                *self.get_register_mut(t) = *self.get_register(a)
                            }
                            Parameter::Value(v) => *self.get_register_mut(t) = v,
                        }
                    }
                    self.instruction_ptr += 1;
                }
                Instruction::INC(p1) => {
                    if let Parameter::Address(a) = p1 {
                        *self.get_register_mut(a) += 1;
                    }
                    self.instruction_ptr += 1;
                }
                Instruction::DEC(p1) => {
                    if let Parameter::Address(a) = p1 {
                        *self.get_register_mut(a) -= 1;
                    }
                    self.instruction_ptr += 1;
                }
                Instruction::JNZ(p1, p2) => {
                    let v = match p1 {
                        Parameter::Address(a) => *self.get_register(a),
                        Parameter::Value(v) => v,
                    };

                    let jmp = match p2 {
                        Parameter::Address(a) => *self.get_register(a),
                        Parameter::Value(v) => v,
                    };

                    if (v) != 0 {
                        self.instruction_ptr = (self.instruction_ptr as i64 + jmp) as usize;
                    } else {
                        self.instruction_ptr += 1;
                    }
                }
                Instruction::TGL(p1) => {
                    if let Parameter::Address(r) = p1 {
                        let index = (self.get_register(r) + (self.instruction_ptr as i64)) as usize;

                        if index < code.len() {
                            code[index] = match code[index] {
                                Instruction::CPY(p1, p2) => Instruction::JNZ(p1, p2),
                                Instruction::JNZ(p1, p2) => Instruction::CPY(p1, p2),
                                Instruction::DEC(p1) => Instruction::INC(p1),
                                Instruction::INC(p1) => Instruction::DEC(p1),
                                Instruction::TGL(p1) => Instruction::INC(p1),
                                Instruction::OUT(p1) => Instruction::INC(p1),
                            };
                        }
                    }
                    self.instruction_ptr += 1;
                }
                Instruction::OUT(p1) => match p1 {
                    Parameter::Address(r) => return Some(*self.get_register(r)),
                    Parameter::Value(v) => return Some(v),
                },
            }
        }

        None
    }

    pub fn get_register(&self, register: char) -> &Literal {
        let index = register as usize - 'a' as usize;
        &self.registers[index]
    }

    pub fn get_register_mut(&mut self, register: char) -> &mut Literal {
        let index = register as usize - 'a' as usize;
        &mut self.registers[index]
    }
}
