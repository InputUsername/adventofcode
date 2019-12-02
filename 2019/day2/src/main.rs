use std::fs;
use std::convert::TryInto;

fn main() {
    let input: Vec<u32> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    part1(input.clone());
    part2(input.clone());
}

fn interpret(mem: &mut [u32]) {
    let mut pc = 0;
    loop {
        let opcode = mem[pc];
        match opcode {
            1 | 2 => {
                let a: usize = mem[pc + 1].try_into().unwrap();
                let b: usize = mem[pc + 2].try_into().unwrap();
                let dest: usize = mem[pc + 3].try_into().unwrap();

                if opcode == 1 {
                    mem[dest] = mem[a] + mem[b];
                } else if opcode == 2 {
                    mem[dest] = mem[a] * mem[b];
                }
            }
            99 => break,
            _ => panic!("Unknown opcode {}", mem[pc]),
        }
        pc += 4;
    }
}

fn part1(mut input: Vec<u32>) {
    input[1] = 12;
    input[2] = 2;

    interpret(&mut input);

    println!("{}", input[0]);
}

fn part2(input: Vec<u32>) {
    let mut mem = vec![0; input.len()];

    let mut done = false;
    for noun in 0..99 {
        for verb in 0..99 {
            mem.clone_from(&input);
            mem[1] = noun;
            mem[2] = verb;

            interpret(&mut mem);

            if mem[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                done = true;
                break;
            }
        }

        if done {
            break;
        }
    }
}