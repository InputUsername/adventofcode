use std::fs;

use intcode::{Computer, InterpretStep};

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

fn swap(slice: &mut [i64], i: usize, j: usize) {
    let tmp = slice[i];
    slice[i] = slice[j];
    slice[j] = tmp;
}

fn permutations(cpu: &Computer, setting: &mut [i64], f: fn(&Computer, &[i64]) -> i64, i: usize, j: usize) -> i64 {
    if i == j {
        return f(cpu, setting);
    }

    let mut max_output = 0;
    for k in i..=j {
        swap(setting, k, i);
        let output = permutations(cpu, setting, f, i + 1, j);
        if output > max_output {
            max_output = output;
        }
        swap(setting, k, i);
    }

    return max_output;
}

fn try_permutation(cpu: &Computer, setting: &[i64]) -> i64 {
    let mut input = 0;
    for &n in setting {
        let mut amp = cpu.clone();
        let outputs = amp.run(&[n, input]);
        input = outputs[0];
    }
    input
}

/// Partially Interpret an Intcode program up to the next output or halt instruction
fn interpret_partial(cpu: &mut Computer, input: i64) -> Option<i64> {
    loop {
        match cpu.step(Some(input)) {
            InterpretStep::Output(output) => return Some(output),
            InterpretStep::Halt => return None,
            _ => {}
        }
    }
}

fn try_feedback_loop(cpu: &Computer, setting: &[i64]) -> i64 {
    let mut amps = Vec::new();
    for &n in setting {
        let mut amp = cpu.clone();
        let _ = amp.step(Some(n));
        amps.push(amp);
    }

    let mut input = 0;
    loop {
        for amp in amps.iter_mut() {
            if let Some(output) = interpret_partial(amp, input) {
                input = output;
            } else {
                // If the first amp halts, all amps will halt after it and "input" will contain the output
                // of the last amp in the previous loop
                return input;
            }
        }
    }
}

fn part1(cpu: Computer) {
    let mut setting: Vec<i64> = (0..=4).collect();
    let len = setting.len();
    let max_output = permutations(&cpu, &mut setting, try_permutation, 0, len - 1);

    println!("{}", max_output);
}

fn part2(cpu: Computer) {
    let mut setting: Vec<i64> = (5..=9).collect();
    let len = setting.len();
    let max_output = permutations(&cpu, &mut setting, try_feedback_loop, 0, len - 1);

    println!("{}", max_output);
}