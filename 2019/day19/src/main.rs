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
}

fn check(cpu: &Computer, x: i64, y: i64) -> bool {
    let mut program = cpu.clone();
    let outputs = program.run(&[x, y]);
    outputs[0] == 1
}

fn part1(cpu: Computer) {
    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            if check(&cpu, x, y) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}