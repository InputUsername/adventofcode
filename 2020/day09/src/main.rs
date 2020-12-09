use std::fs;

fn find_breaking(input: &[i64]) -> i64 {
    input.windows(26)
        .find(|&w| {
            for (i, a) in w.iter().enumerate() {
                for b in w[i..].iter() {
                    if a + b == w[w.len() - 1] {
                        return false;
                    }
                }
            }
            return true;
        })
        .map(|w| w[w.len() - 1])
        .unwrap()
}

fn part1(input: &[i64]) {
    println!("{}", find_breaking(input));
}

fn min_max(w: &[i64]) -> (i64, i64) {
    let i = w[0];
    w[1..].iter().fold((i, i), |(min, max), &val| {
        (min.min(val), max.max(val))
    })
}

fn part2(input: &[i64]) {
    let n = find_breaking(input);
    'outer: for len in 2..input.len() {
        for w in input.windows(len) {
            if w.iter().sum::<i64>() == n {
                let (min, max) = min_max(w);
                println!("{}", min + max);
                break 'outer;
            }
        }
    }
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input").unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}
