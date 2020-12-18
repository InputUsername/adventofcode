use std::collections::HashMap;

fn play(input: &[i32], final_turn: usize) -> i32 {
    let mut seen: HashMap<i32, usize> = input[..input.len()-1].iter().enumerate()
        .map(|(t, &n)| (n, t))
        .collect();

    let mut last_spoken = *input.last().unwrap();
    for turn in input.len()..final_turn {
        if let Some(&t) = seen.get(&last_spoken) {
            let diff = turn - 1 - t;
            seen.insert(last_spoken, turn - 1);
            last_spoken = diff as i32;
        } else {
            seen.insert(last_spoken, turn - 1);
            last_spoken = 0;
        }
    }

    last_spoken
}

fn part1(input: &[i32]) {
    println!("{}", play(input, 2020));
}

fn part2(input: &[i32]) {
    println!("{}", play(input, 30_000_000));
}

fn main() {
    let input = [7, 14, 0, 17, 11, 1, 2];

    part1(&input);
    part2(&input);
}
