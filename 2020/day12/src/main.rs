use std::fs;

enum Instr {
    N, S, E, W,
    L, R, F,
}

struct Move(Instr, i32);

fn parse(s: &str) -> Vec<Move> {
    s.lines()
        .map(|l| {
            use Instr::*;

            let n = l[1..].parse().unwrap();
            let i = match &l[..1] {
                "N" => N,
                "S" => S,
                "E" => E,
                "W" => W,
                "L" => L,
                "R" => R,
                "F" => F,
                _ => unreachable!(),
            };

            Move(i, n)
        })
        .collect()
}

struct Ferry {
    x: i32,
    y: i32,
    dir: i32,
}

impl Ferry {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: 90,
        }
    }

    fn turn(&mut self, angle: i32) {
        self.dir += angle;
        if self.dir < 0 {
            self.dir += 360;
        }
        if self.dir >= 360 {
            self.dir -= 360;
        }
    }

    fn step(&mut self, m: &Move) {
        use Instr::*;

        match m.0 {
            N => self.y += m.1,
            S => self.y -= m.1,
            E => self.x += m.1,
            W => self.x -= m.1,
            L => self.turn(-m.1),
            R => self.turn(m.1),
            F => {
                match self.dir {
                    0 => self.y += m.1,
                    90 => self.x += m.1,
                    180 => self.y -= m.1,
                    270 => self.x -= m.1,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn part1(input: &[Move]) {
    let mut f = Ferry::new();
    for m in input.iter() {
        f.step(m);
    }
    println!("{}", f.manhattan_distance());
}

struct WaypointFerry {
    ferry: Ferry,
    dx: i32,
    dy: i32,
}

impl WaypointFerry {
    fn new() -> Self {
        Self {
            ferry: Ferry::new(),
            dx: 10,
            dy: 1,
        }
    }

    fn move_ferry(&mut self, n: i32) {
        self.ferry.x += n * self.dx;
        self.ferry.y += n * self.dy;
    }

    fn turn(&mut self, mut angle: i32) {
        if angle < 0 {
            angle = 360 + angle;
        }

        for _ in (0..angle).step_by(90) {
            std::mem::swap(&mut self.dx, &mut self.dy);
            self.dy = -self.dy;
        }
    }

    fn step(&mut self, m: &Move) {
        use Instr::*;

        match m.0 {
            N => self.dy += m.1,
            S => self.dy -= m.1,
            E => self.dx += m.1,
            W => self.dx -= m.1,
            L => self.turn(-m.1),
            R => self.turn(m.1),
            F => self.move_ferry(m.1),
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.ferry.manhattan_distance()
    }
}

fn part2(input: &[Move]) {
    let mut wf = WaypointFerry::new();
    for m in input.iter() {
        wf.step(m);
    }
    println!("{}", wf.manhattan_distance());
}

fn main() {
    let moves = parse(&fs::read_to_string("input").unwrap());

    part1(&moves);
    part2(&moves);
}
