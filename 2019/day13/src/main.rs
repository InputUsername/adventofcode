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
    part2(cpu.clone());
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

fn part2(mut cpu: Computer) {
    cpu.memory[0] = 2;

    let mut score = 0;
    let mut ball = (0, 0);
    let mut paddle = (0, 0);
    let mut input = None;
    let mut x = None;
    let mut y = None;

    loop {
        if cpu.wants_input() {
            if ball.0 < paddle.0 {
                input.replace(-1);
            } else if ball.0 == paddle.0 {
                input.replace(0);
            } else if ball.0 > paddle.0 {
                input.replace(1);
            }
        }

        match cpu.step(input) {
            InterpretStep::Output(n) => {
                if x.is_none() {
                    x.replace(n);
                } else if y.is_none() {
                    y.replace(n);
                } else {
                    let xx = x.unwrap();
                    let yy = y.unwrap();

                    if xx == -1 && yy == 0 {
                        score = n;
                    } else if n == 3 {
                        paddle = (xx, yy);
                    } else if n == 4 {
                        ball = (xx, yy);
                    }

                    let _ = x.take();
                    let _ = y.take();
                }
            }
            InterpretStep::Halt => break,
            _ => {}
        }
    }

    println!("{}", score);
}
