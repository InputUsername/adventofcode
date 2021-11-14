use std::collections::{HashMap, HashSet};

enum Command {
    On,
    Toggle,
    Off,
}

struct Instruction {
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
    command: Command,
}

const ON: &str = "turn on ";
const TOGGLE: &str = "toggle ";
const OFF: &str = "turn off ";

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let command = if s.starts_with(ON) {
            s = s.trim_start_matches(ON);
            Command::On
        } else if s.starts_with(TOGGLE) {
            s = s.trim_start_matches(TOGGLE);
            Command::Toggle
        } else if s.starts_with(OFF) {
            s = s.trim_start_matches(OFF);
            Command::Off
        } else {
            unreachable!()
        };
        let mut parts = s.split(" through ");

        let mut coord0 = parts.next().unwrap().split(',');
        let x0 = coord0.next().unwrap().parse().unwrap();
        let y0 = coord0.next().unwrap().parse().unwrap();

        let mut coord1 = parts.next().unwrap().split(',');
        let x1 = coord1.next().unwrap().parse().unwrap();
        let y1 = coord1.next().unwrap().parse().unwrap();

        Ok(Self {
            x0,
            y0,
            x1,
            y1,
            command,
        })
    }
}

fn main() {
    let input: Vec<Instruction> = std::fs::read_to_string("input").unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Instruction]) {
    let mut grid = HashSet::new();

    for instr in input.iter() {
        for x in instr.x0..=instr.x1 {
            for y in instr.y0..=instr.y1 {
                match instr.command {
                    Command::On => {
                        grid.insert((x, y));
                    }
                    Command::Toggle => {
                        if grid.contains(&(x, y)) {
                            grid.remove(&(x, y));
                        } else {
                            grid.insert((x, y));
                        }
                    }
                    Command::Off => {
                        grid.remove(&(x, y));
                    }
                }
            }
        }
    }

    println!("{}", grid.len());
}

fn part2(input: &[Instruction]) {
    let mut grid: HashMap<_, u32> = HashMap::new();

    for instr in input.iter() {
        for x in instr.x0..=instr.x1 {
            for y in instr.y0..=instr.y1 {
                match instr.command {
                    Command::On => *grid.entry((x, y)).or_insert(0) += 1,
                    Command::Toggle => *grid.entry((x, y)).or_insert(0) += 2,
                    Command::Off => {
                        let e = grid.entry((x, y)).or_insert(0);
                        *e = e.saturating_sub(1);
                    }
                }
            }
        }
    }

    let total: u32 = grid.values().sum();
    println!("{}", total);
}
