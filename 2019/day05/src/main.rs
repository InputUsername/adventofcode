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
    let outputs = cpu.run(&[1]);

    println!("{}", outputs[outputs.len() - 1]);
}

fn part2(mut cpu: Computer) {
    let outputs = cpu.run(&[5]);

    println!("{}", outputs[0]);
}