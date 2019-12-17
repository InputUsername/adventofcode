use std::fs;
use std::collections::HashMap;

use intcode::{Computer, InterpretStep};

fn main() {
    let input: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let cpu = Computer::from(&input[..]);

    part1(cpu.clone());
}

fn get_output(cpu: &mut Computer) -> Option<(i64, i64, i64)> {
    let mut out = [-1; 3];
    let mut i = 0;

    loop {
        match cpu.step(None) {
            InterpretStep::Output(output) => {
                out[i] = output;
                i += 1;

                if i == 3 {
                    return Some((out[0], out[1], out[2]));
                }
            }
            InterpretStep::Halt => return None,
            _ => {}
        }
    }
}

fn part1(mut cpu: Computer) {
    let mut screen = HashMap::new();

    loop {
        if let Some((x, y, id)) = get_output(&mut cpu) {
            *screen.entry((x, y)).or_insert(0) = id;
        } else {
            break;
        }
    }

    let count = screen.values().filter(|&&id| id == 2).count();
    println!("{}", count);
}
