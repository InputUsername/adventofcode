use std::fs;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instr {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

fn parse_program(s: &str) -> Vec<Instr> {
    s.lines()
        .map(|l| {

            match &l[..3] {
                "acc" => Instr::Acc(l[4..].parse().unwrap()),
                "jmp" => Instr::Jmp(l[4..].parse().unwrap()),
                "nop" => Instr::Nop(l[4..].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

enum RunResult {
    Loop(i32),
    Terminate(i32),
}

fn try_program(program: &[Instr]) -> RunResult {
    let mut acc = 0;
    let mut pc: isize = 0;
    let mut executed = HashSet::new();
    loop {
        if (pc as usize) >= program.len() {
            return RunResult::Terminate(acc);
        }
        if !executed.insert(pc) {
            return RunResult::Loop(acc);
        }

        match program[pc as usize] {
            Instr::Acc(n) => acc += n,
            Instr::Jmp(n) => {
                pc += n;
                continue;
            }
            Instr::Nop(_) => {}
        }

        pc += 1;
    }
}

fn part1(program: &[Instr]) {
    if let RunResult::Loop(acc) = try_program(program) {
        println!("{}", acc);
    }
}

fn part2(program: &[Instr]) {
    for (idx, i) in program.iter().enumerate() {
        let mut program = program.to_owned();
        match i {
            Instr::Acc(_) => continue,
            Instr::Jmp(n) => program[idx] = Instr::Nop(*n),
            Instr::Nop(n) => program[idx] = Instr::Jmp(*n),
        }
        if let RunResult::Terminate(acc) = try_program(&program) {
            println!("{}", acc);
            break;
        }
    }
}

fn main() {
    let program = parse_program(&fs::read_to_string("input").unwrap());

    part1(&program);
    part2(&program);
}
