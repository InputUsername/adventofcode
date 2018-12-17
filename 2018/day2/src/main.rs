use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut f = File::open("input")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let mut count2 = 0;
    let mut count3 = 0;
    let mut chars: HashMap<char, usize> = HashMap::new();
    for line in input.lines() {
        for c in line.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }
        if chars.values().any(|&c| c == 2) { count2 += 1; }
        if chars.values().any(|&c| c == 3) { count3 += 1; }
        chars.clear();
    }
    println!("{}", count2*count3);
}

fn part2(input: &str) {
    for first in input.lines() {
        for second in input.lines() {
            let common: String = first.chars()
                .zip(second.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect();
            if common.len() == first.len() - 1 {
                println!("{}", common);
                return;
            }
        }
    }
}