use std::fs;
use std::collections::HashMap;

fn part1(input: &[i32]) {
    let (d1, d3) = input.windows(2).map(|w| w[1] - w[0])
        .fold((0, 0), |(d1, d3), d| {
            match d {
                1 => (d1 + 1, d3),
                3 => (d1, d3 + 1),
                _ => (d1, d3),
            }
        });
    println!("{}", d1 * d3);
}

/// Memoized recursive arrange function
fn arrange(
    mem: &mut HashMap<(usize, usize), u64>,
    adapters: &mut [i32],
    i: usize,
    prev: usize
) -> u64
{
    if let Some(&c) = mem.get(&(i, prev)) {
        return c;
    }

    if i == adapters.len() - 1 {
        return 1;
    }

    let mut count = arrange(mem, adapters, i + 1, i);

    // Remove the current index if the gap between
    // the next and the previous used index is not
    // too big
    let diff = adapters[i + 1] - adapters[prev];
    if diff >= 1 && diff <= 3 {
        let p = adapters[i];
        adapters[i] = i32::MIN;
        count += arrange(mem, adapters, i + 1, prev);
        adapters[i] = p;
    }

    mem.insert((i, prev), count);

    count
}

fn part2(input: &[i32]) {
    let count = arrange(
        &mut HashMap::new(),
        &mut input.to_owned(),
        1,
        0
    );
    println!("{}", count);
}

fn main() {
    let mut ratings: Vec<i32> = fs::read_to_string("input").unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    ratings.push(0);
    ratings.sort();
    ratings.push(ratings.last().unwrap() + 3);

    part1(&ratings);
    part2(&ratings);
}
