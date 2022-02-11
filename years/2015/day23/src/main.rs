#[derive(Debug)]
enum Register {
    A,
    B
}

impl From<&str> for Register {
    fn from(id: &str) -> Self {
        match id.chars().next() {
            Some('a') => Self::A,
            Some('b') => Self::B,
            Some(id) => panic!("bad register id : {}", id),
            _ => panic!("empty string")
        }
    }
}

/* ---------- */

#[derive(Debug)]
enum Offset {
    Forward(usize),
    Backward(usize)
}

impl From<&str> for Offset {
    fn from(off_str: &str) -> Self {
        let off = off_str[1..].parse::<usize>().expect("a valid offset number");

        match off_str.chars().next() {
            Some('+') => Self::Forward(off),
            Some('-') => Self::Backward(off),
            Some(sign) => panic!("bad offset : {}", sign),
            _ => panic!("empty string")
        }
    }
}

/* ---------- */

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Increment(Register),
    Triple(Register),
    Jump(Offset),
    JumpIfEven(Register, Offset),
    JumpIfOne(Register, Offset)
}

/* ---------- */

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl Program {
    fn compile(input: &'static str) -> Self {
        let instructions = input.lines().map(|line| {
                let parts = line.split(' ').collect::<Vec<&'static str>>();

                match parts[0] {
                    "hlf" => Instruction::Half(Register::from(parts[1])),
                    "inc" => Instruction::Increment(Register::from(parts[1])),
                    "tpl" => Instruction::Triple(Register::from(parts[1])),
                    "jmp" => Instruction::Jump(Offset::from(parts[1])),
                    "jie" => Instruction::JumpIfEven(Register::from(parts[1]), Offset::from(parts[2])),
                    "jio" => Instruction::JumpIfOne(Register::from(parts[1]), Offset::from(parts[2])),
                    _ => panic!("failed to parse {} as instruction", line)
                }
            })
            .collect();

        Self(instructions)
    }

    fn instruction_at(&self, idx: usize) -> Option<&Instruction> {
        self.0.get(idx)
    }
}

/* ---------- */

#[derive(Debug, Default)]
struct Computer {
    ra: usize,
    rb: usize,
    instr_counter: usize
}

impl Computer {
    #[inline(always)]
    fn new() -> Self {
        Self::default()
    }

    fn breg(&self) -> usize {
        self.rb
    }

    fn run(&mut self, prog: &Program) {
        while let Some(instr) = prog.instruction_at(self.instr_counter) {
            match instr {
                Instruction::Half(reg) => self.half(reg),
                Instruction::Increment(reg) => self.increment(reg),
                Instruction::Triple(reg) => self.triple(reg),
                Instruction::Jump(off) => self.jump(off),
                Instruction::JumpIfEven(reg, off) => self.jump_if_even(reg, off),
                Instruction::JumpIfOne(reg, off) => self.jump_if_one(reg, off),
            }
        }
    }

    fn half(&mut self, reg: &Register) {
        match reg {
            Register::A => self.ra /= 2,
            Register::B => self.rb /= 2
        }

        self.instr_counter += 1;
    }

    fn increment(&mut self, reg: &Register) {
        match reg {
            Register::A => self.ra += 1,
            Register::B => self.rb += 1
        }

        self.instr_counter += 1;
    }

    fn triple(&mut self, reg: &Register) {
        match reg {
            Register::A => self.ra *= 3,
            Register::B => self.rb *= 3
        }

        self.instr_counter += 1;
    }

    fn jump(&mut self, off: &Offset) {
        match off {
            Offset::Forward(val) => self.instr_counter += val,
            Offset::Backward(val) => self.instr_counter -= val
        }
    }

    fn jump_if_even(&mut self, reg: &Register, off: &Offset) {
        let reg_val = match reg {
            Register::A => self.ra,
            Register::B => self.rb
        };

        if reg_val % 2 != 0 {
            self.instr_counter += 1;
            return;
        }

        match off {
            Offset::Forward(val) => self.instr_counter += val,
            Offset::Backward(val) => self.instr_counter -= val
        }
    }

    fn jump_if_one(&mut self, reg: &Register, off: &Offset) {
        let reg_val = match reg {
            Register::A => self.ra,
            Register::B => self.rb
        };

        if reg_val != 1 {
            self.instr_counter += 1;
            return;
        }

        match off {
            Offset::Forward(val) => self.instr_counter += val,
            Offset::Backward(val) => self.instr_counter -= val
        }
    }
}

/* ---------- */

fn main() {
    let program = Program::compile(utils::input_str!());
    let mut comp = Computer::new();

    comp.run(&program);

    println!("result = {}", comp.breg());
}
