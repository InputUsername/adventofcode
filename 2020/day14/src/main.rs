use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum MaskBit { X, Zero, One, }

fn parse_mask(s: &str) -> [MaskBit; 36] {
    let mut m = [MaskBit::X; 36];
    for (i, c) in s.char_indices() {
        m[i] = match c {
            'X' => MaskBit::X,
            '0' => MaskBit::Zero,
            '1' => MaskBit::One,
            _ => unreachable!(),
        };
    }
    m
}

fn apply_mask_1(m: &[MaskBit; 36], mut val: u64) -> u64 {
    for (i, b) in m.iter().rev().enumerate() {
        match b {
            MaskBit::Zero => val &= !(1 << i),
            MaskBit::One => val |= 1 << i,
            MaskBit::X => {}
        }
    }
    val
}

#[derive(Debug)]
enum Instruction {
    Mask([MaskBit; 36]),
    Memory(u64, u64),
}

fn parse_program(s: &str) -> Vec<Instruction> {
    s.lines().map(|l| {
        let mut parts = l.split(" = ");
        let (instr, val) = (parts.next().unwrap(), parts.next().unwrap());
        match instr {
            "mask" => Instruction::Mask(parse_mask(val)),
            _ => {
                let addr = instr.trim_start_matches("mem[").trim_end_matches("]")
                    .parse().unwrap();
                let val = val.parse().unwrap();

                Instruction::Memory(addr, val)
            }
        }
    }).collect()
}

fn run_program_1(program: &[Instruction]) -> u64 {
    let mut mask = [MaskBit::X; 36];
    let mut memory = HashMap::new();
    for instr in program.iter() {
        match instr {
            Instruction::Mask(m) => mask = *m,
            Instruction::Memory(addr, val) => {
                let val = apply_mask_1(&mask, *val);
                memory.insert(*addr, val);
            }
        }
    }
    memory.iter().map(|(_, &val)| val).sum()
}

fn part1(input: &[Instruction]) {
    println!("{}", run_program_1(input));
}

// Write value to all addresses according to the bit mask
fn write(m: &[MaskBit; 36], i: usize, addr: u64, memory: &mut HashMap<u64, u64>, val: u64) {
    if i == 36 {
        memory.insert(addr, val);
        return;
    }

    match m[35 - i] {
        MaskBit::Zero => write(m, i + 1, addr, memory, val),
        MaskBit::One => write(m, i + 1, addr | (1 << i), memory, val),
        MaskBit::X => {
            write(m, i + 1, addr & !(1 << i), memory, val);
            write(m, i + 1, addr | (1 << i), memory, val);
        }
    }
}

fn run_program_2(program: &[Instruction]) -> u64 {
    let mut mask = [MaskBit::X; 36];
    let mut memory = HashMap::new();
    for instr in program.iter() {
        match instr {
            Instruction::Mask(m) => mask = *m,
            Instruction::Memory(addr, val) =>
                write(&mask, 0, *addr, &mut memory, *val),
        }
    }
    memory.iter().map(|(_, &val)| val).sum()
}

fn part2(input: &[Instruction]) {
    println!("{}", run_program_2(input));
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let program = parse_program(&input);

    part1(&program);
    part2(&program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask() {
        let m = parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(
            apply_mask(&m, 0b1011),
            0b1001001
        );
        assert_eq!(
            apply_mask(&m, 0b1100101),
            0b1100101
        );
        assert_eq!(
            apply_mask(&m, 0),
            0b1000000
        );
    }
}
