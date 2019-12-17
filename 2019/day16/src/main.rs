use std::fs;

fn main() {
    let input: Vec<u8> = fs::read_to_string("input")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    part1(&input);
}

fn make_pattern(n: usize, pattern: &mut [i8]) {
    const BASE: &[i8] = &[0, 1, 0, -1];
    let mut j = 0;
    for i in 0..pattern.len() {
        pattern[i] = BASE[j];
        if (i + 1) % n == 0 {
            j += 1;
            j %= BASE.len();
        }
    }
}

fn fft(input: &[u8]) -> Vec<u8> {
    let len = input.len();
    let mut output = vec![0; len];
    let mut pattern = vec![0; len + 1];

    for i in 0..len {
        make_pattern(i + 1, &mut pattern);

        let n: i32 = input
            .iter()
            .zip(pattern[1..].iter())
            .map(|(&i, &p)| (i as i32) * (p as i32))
            .sum();

        output[i] = (n.abs() % 10) as u8;
    }

    output
}

fn part1(input: &[u8]) {
    let mut input = input.to_vec();
    for _ in 0..100 {
        input = fft(&input);
    }
    for i in 0..8 {
        print!("{}", input[i]);
    }
    println!();
}