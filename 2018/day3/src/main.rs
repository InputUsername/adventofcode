use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    part1(&input);

    Ok(())
}

fn part1(input: &str) {
    
}