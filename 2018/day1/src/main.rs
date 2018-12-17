use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut f = File::open("input")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &str) {
    let freq = input.lines()
        .fold(0, |a, l| a + l.parse::<i32>().unwrap());

    println!("{}", freq);
}

fn part2(input: &str) {
    let mut freqs: HashSet<i32> = HashSet::new();
    let mut freq = 0;
    loop {
        for line in input.lines() {
            freqs.insert(freq);
            freq += line.parse::<i32>().unwrap();
            if freqs.contains(&freq) {
                println!("{}", freq);
                return;
            }
        }
    }
}