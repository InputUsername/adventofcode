use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Moon {
    id: usize,
    pos: [i64; 3],
    vel: [i64; 3],
}

fn sign(n: i64) -> i64 {
    if n < 0 { -1 }
    else if n > 0 { 1 }
    else { 0 }
}

impl Moon {
    fn gravity(&self, other: &Moon) -> [i64; 3] {
        let mut grav = [0; 3];
        for i in 0..3 {
            grav[i] = sign(other.pos[i] - self.pos[i]);
        }
        grav
    }

    fn energy(&self) -> i64 {
        let pot: i64 = self.pos.iter().map(|n| n.abs()).sum();
        let kin: i64 = self.vel.iter().map(|n| n.abs()).sum();
        pot * kin
    }
}

type AxisState = [[i64; 2]; 4];

#[derive(Clone)]
struct System {
    moons: Vec<Moon>,
}

impl System {
    fn new(input: &str) -> Self {
        let moons = input.lines()
            .enumerate()
            .map(|(i, line)| {
                let line = line.trim();
                let mut coords = line[1..line.len()-1].split(", ")
                    .map(|coord| coord[2..].parse().unwrap());
                let (x, y, z) = (
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap()
                );
                Moon {
                    id: i,
                    pos: [x, y, z],
                    vel: [0, 0, 0],
                }
            })
            .collect();

        Self { moons }
    }

    fn step(&mut self) {
        let mut gravity = HashMap::new();

        for a in self.moons.iter() {
            for b in self.moons.iter() {
                if a.id == b.id { continue; }

                let grav = a.gravity(b);
                let entry = gravity.entry(a.id).or_insert([0; 3]);
                for i in 0..3 {
                    entry[i] += grav[i];
                }
            }
        }

        for moon in self.moons.iter_mut() {
            let grav = gravity.remove(&moon.id).unwrap();
            for i in 0..3 {
                moon.vel[i] += grav[i];
                moon.pos[i] += moon.vel[i];
            }
        }
    }

    fn energy(&self) -> i64 {
        self.moons.iter().map(|moon| moon.energy()).sum()
    }

    fn axis_state(&self, axis: usize) -> AxisState {
        let mut state = [[0; 2]; 4];
        for i in 0..4 {
            state[i] = [self.moons[i].pos[axis], self.moons[i].vel[axis]];
        }
        state
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

fn lcm(a: u64, b: u64, c: u64) -> u64 {
    let x = (a * b) / gcd(a, b);
    (c * x) / gcd(c, x)
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let system = System::new(&input);

    part1(system.clone());
    part2(system.clone());
}

fn part1(mut system: System) {
    for _ in 0..1000 {
        system.step();
    }

    println!("{}", system.energy());
}

fn part2(mut system: System) {
    let mut states = [HashSet::new(), HashSet::new(), HashSet::new()];
    let mut steps: [u64; 3] = [0; 3];
    let mut rep = [false; 3];

    for i in 0..3 {
        states[i].insert(system.axis_state(i));
    }

    loop {
        system.step();

        for i in 0..3 {
            if !rep[i] {
                steps[i] += 1;

                let state = system.axis_state(i);

                if states[i].contains(&state) {
                    rep[i] = true;
                } else {
                    states[i].insert(state);
                }
            }
        }

        if rep[0] && rep[1] && rep[2] {
            break;
        }
    }

    let total_steps = lcm(steps[0], steps[1], steps[2]);

    println!("{}", total_steps);
}