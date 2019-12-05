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
    let output = intcode::interpret(&mut input, &[String::from("1")]);

    println!("{}", output[output.len() - 1]);
}

fn part2(mut input: Vec<i64>) {
    let output = intcode::interpret(&mut input, &[String::from("5")]);

    println!("{}", output[0]);
}