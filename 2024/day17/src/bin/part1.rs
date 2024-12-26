use common::Itertools;

fn main() {
    let input: String = common::AocInput::fetch(2024, 17).unwrap().into();
    println!("Output: {}", process(&input));
}

struct Computer {
    a: u32,
    b: u32,
    c: u32,
}

#[repr(u32)]
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

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
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
    fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }

    fn evaluate(&mut self, program: &[u32]) -> Vec<u32> {
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
                    let dem = 2u32.pow(self.combo(operand));
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
                    let dem = 2u32.pow(self.combo(operand));
                    self.b = self.a / dem;
                    pointer += 2;
                }
                Instruction::Cdv => {
                    let dem = 2u32.pow(self.combo(operand));
                    self.c = self.a / dem;
                    pointer += 2;
                }
            }
        }
        out
    }

    fn combo(&self, opcode: u32) -> u32 {
        match opcode {
            0..=3 => opcode,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("unsupported opcode: {opcode}"),
        }
    }
}

fn process(input: &str) -> String {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let (a, b, c) = registers
        .lines()
        .map(|l| {
            let (_, value) = l.split_once(": ").unwrap();
            value.parse::<u32>().unwrap()
        })
        .collect_tuple()
        .unwrap();

    let (_, program) = program.split_once(": ").unwrap();

    let program: Vec<u32> = program
        .trim()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect_vec();

    let mut computer = Computer::new(a, b, c);

    computer.evaluate(&program).into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_processes() {
        assert_eq!(process(include_str!("./sample.txt")), "4,6,3,5,6,3,5,2,1,0");
    }
}
