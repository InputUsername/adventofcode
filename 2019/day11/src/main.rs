use std::fs;
use std::collections::HashMap;

use intcode::{Computer, InterpretStep};

enum Dir { N, E, S, W, }

impl Dir {
    fn turn(&mut self, n: i64) -> (i64, i64) {
        *self = if n == 0 {
            match self {
                Dir::N => Dir::W,
                Dir::E => Dir::N,
                Dir::S => Dir::E,
                Dir::W => Dir::S,
            }
        } else {
            match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
            }
        };

        match self {
            Dir::N => (0, -1),
            Dir::E => (1, 0),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
        }
    }
}

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

fn run_cycle(cpu: &mut Computer, input: i64) -> Option<(i64, i64)> {
    let mut i = 0;
    let mut outputs = [None; 2];
    loop {
        match cpu.step(Some(input)) {
            InterpretStep::Output(output) => {
                outputs[i].replace(output);
                i += 1;
                if i == 2 {
                    break;
                }
            }
            InterpretStep::Halt => return None,
            _ => continue,
        }
    }
    Some((outputs[0].unwrap(), outputs[1].unwrap()))
}

fn run_robot(cpu: &mut Computer, initial_color: i64) -> HashMap<(i64, i64), i64> {
    let mut hull = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::N;

    hull.insert((0, 0), initial_color);

    loop {
        let color = hull.entry((x, y)).or_insert(0);
        if let Some((paint, turn)) = run_cycle(cpu, *color) {
            *color = paint;
            let (dx, dy) = dir.turn(turn);
            x += dx;
            y += dy;
        } else {
            break;
        }
    }

    hull
}

fn part1(mut cpu: Computer) {
    let hull = run_robot(&mut cpu, 0);

    println!("{}", hull.len());
}

fn part2(mut cpu: Computer) {
    let hull = run_robot(&mut cpu, 1);

    let ((x0, _), _) = hull.iter().min_by_key(|((x, _), _)| x).unwrap();
    let ((_, y0), _) = hull.iter().min_by_key(|((_, y), _)| y).unwrap();
    let ((x1, _), _) = hull.iter().max_by_key(|((x, _), _)| x).unwrap();
    let ((_, y1), _) = hull.iter().max_by_key(|((_, y), _)| y).unwrap();

    for y in *y0..=*y1 {
        for x in *x0..=*x1 {
            match hull.get(&(x, y)).unwrap_or(&0) {
                0 => print!("."),
                1 => print!("#"),
                _ => {},
            }
        }
        println!();
    }
}