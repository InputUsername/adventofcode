use std::fs;

use intcode;

fn main() {
    let input: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    part1(&input);
}

fn swap(slice: &mut [i64], i: usize, j: usize) {
    let tmp = slice[i];
    slice[i] = slice[j];
    slice[j] = tmp;
}

fn permutations(input: &[i64], setting: &mut [i64], f: fn(&[i64], &[i64]) -> i64, i: usize, j: usize) -> i64 {
    if i == j {
        return f(input, setting);
    }

    let mut max_output = 0;
    for k in i..=j {
        swap(setting, k, i);
        let output = permutations(input, setting, f, i + 1, j);
        if output > max_output {
            max_output = output;
        }
        swap(setting, k, i);
    }

    return max_output;
}

fn try_permutation(input: &[i64], setting: &[i64]) -> i64 {
    let mut mem: Vec<i64> = input.to_vec();
    let mut outputs = intcode::interpret(&mut mem, &[setting[0].to_string(), "0".to_string()]);
    let mut output = outputs.pop().unwrap();
    for n in &setting[1..] {
        mem.copy_from_slice(input);
        outputs = intcode::interpret(&mut mem, &[n.to_string(), output]);
        output = outputs.pop().unwrap();
    }
    output.parse().unwrap()
}

fn part1(input: &[i64]) {
    let mut setting: Vec<i64> = (0..=4).collect();
    let len = setting.len();
    let max_output = permutations(input, &mut setting, try_permutation, 0, len - 1);

    println!("{}", max_output);
}