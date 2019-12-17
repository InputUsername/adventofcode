use std::fs;

use intcode::Computer;

fn main() {
    let input: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let cpu = Computer::from(&input[..]);

    part1(cpu.clone());
    part2(cpu.clone());
}

fn part1(mut cpu: Computer) {
    cpu.memory[1] = 12;
    cpu.memory[2] = 2;

    let _ = cpu.run(&[]);

    println!("{}", cpu.memory[0]);
}

fn part2(cpu: Computer) {
    const VALUE: i64 = 19690720;

    let mut done = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut cpu = cpu.clone();
            cpu.memory[1] = noun;
            cpu.memory[2] = verb;
            let _ = cpu.run(&[]);
            if cpu.memory[0] == VALUE {
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