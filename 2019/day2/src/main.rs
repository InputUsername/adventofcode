use std::fs;

use intcode;

fn main() {
    let input: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    part1(input.clone());
    part2(input.clone());
}

fn part1(mut input: Vec<i64>) {
    input[1] = 12;
    input[2] = 2;

    intcode::interpret(&mut input);

    println!("{}", input[0]);
}

fn part2(input: Vec<i64>) {
    let mut mem = vec![0; input.len()];

    let mut done = false;
    for noun in 0..99 {
        for verb in 0..99 {
            mem.clone_from(&input);
            mem[1] = noun;
            mem[2] = verb;

            intcode::interpret(&mut mem);

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