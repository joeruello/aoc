use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 17).unwrap().into();
    println!("Output: {}", process(&input));
}

#[derive(Debug)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
}

#[repr(u64)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("invalid opcode {value}"),
        }
    }
}

impl Computer {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }

    fn evaluate(&mut self, program: &[u64]) -> Vec<u64> {
        let mut pointer = 0;
        let mut out = vec![];
        loop {
            if pointer >= program.len() {
                break;
            }

            let instruction: Instruction = program[pointer].into();
            let operand = program[pointer + 1];

            match instruction {
                Instruction::Adv => {
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.a /= dem;
                    pointer += 2;
                }
                Instruction::Bxl => {
                    self.b ^= operand;
                    pointer += 2;
                }
                Instruction::Bst => {
                    self.b = self.combo(operand) % 8;
                    pointer += 2;
                }
                Instruction::Jnz => {
                    if self.a == 0 {
                        pointer += 2
                    } else {
                        pointer = operand as usize
                    }
                }
                Instruction::Bxc => {
                    self.b ^= self.c;
                    pointer += 2;
                }
                Instruction::Out => {
                    out.push(self.combo(operand) % 8);

                    pointer += 2;
                }
                Instruction::Bdv => {
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.b = self.a / dem;
                    pointer += 2;
                }
                Instruction::Cdv => {
                    let dem = 2u64.pow(self.combo(operand) as u32);
                    self.c = self.a / dem;
                    pointer += 2;
                }
            }
        }
        out
    }

    fn combo(&self, opcode: u64) -> u64 {
        match opcode {
            0..=3 => opcode,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("unsupported opcode: {opcode}"),
        }
    }
}

fn process(input: &str) -> u64 {
    let (_, program) = input.split_once("\n\n").unwrap();
    let (_, program) = program.split_once(": ").unwrap();

    let program: Vec<u64> = program
        .trim()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect_vec();

    let mut idx = program.len() - 1;
    let mut to_find = program.clone().into_iter().skip(idx).collect_vec();
    let mut n = 0;
    loop {
        let mut computer = Computer::new(n, 0, 0);
        let res = computer.evaluate(&program);
        if res == program {
            return n;
        } else if res.eq(&to_find) {
            idx -= 1;
            to_find = program.clone().into_iter().skip(idx).collect_vec();
            n *= 8;
        } else {
            n += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample2.txt")), 117440);
    }
}
