use std::fs;

use intcode::Computer;

fn main() {
    let mem: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let cpu = Computer::from(&mem[..]);

    part1(cpu.clone());
    part2(cpu.clone());
}

fn part1(mut cpu: Computer) {
    let outputs = cpu.run(&[1]);
    println!("{}", outputs[0]);
}

fn part2(mut cpu: Computer) {
    let outputs = cpu.run(&[2]);
    println!("{}", outputs[0]);
}