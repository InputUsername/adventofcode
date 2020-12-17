use std::fs;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Taken,
}

#[derive(Clone)]
struct SeatingArea {
    cells: Vec<Cell>,
    w: usize,
    h: usize,
}

impl SeatingArea {
    fn from_str(s: &str) -> Self {
        let mut w = 0;
        let cells: Vec<_> = s
            .lines()
            .flat_map(|l| {
                w = l.len();
                l.chars().map(|c| match c {
                    '.' => Cell::Floor,
                    'L' => Cell::Empty,
                    '#' => Cell::Taken,
                    _ => unreachable!(),
                })
            }).collect();
    
        let h = cells.len() / w;
    
        Self { cells, w, h }
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.w + x]
    }

    fn set(&mut self, x: usize, y: usize, c: Cell) {
        self.cells[y * self.w + x] = c;
    }

    fn check_dir(&self, x: isize, y: isize, dx: isize, dy: isize, dist: usize) -> bool {
        if dx == 0 && dy == 0 {
            return false;
        }

        let mut cx = x;
        let mut cy = y;
        let mut steps = 0;
        loop {
            cx += dx;
            cy += dy;
            steps += 1;

            if cx < 0 || usize::try_from(cx).unwrap() >= self.w
            || cy < 0 || usize::try_from(cy).unwrap() >= self.h {
                return false;
            }

            let cell = self.get(
                usize::try_from(cx).unwrap(),
                usize::try_from(cy).unwrap(),
            );

            if dist != 0 && steps == dist {
                return cell == Cell::Taken;
            }

            match cell {
                Cell::Empty => return false,
                Cell::Taken => return true,
                Cell::Floor => continue,
            }
        }
    }

    fn count(&self, x: usize, y: usize, dist: usize) -> i32 {
        let x = isize::try_from(x).unwrap();
        let y = isize::try_from(y).unwrap();
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if self.check_dir(x, y, dx, dy, dist) {
                    count += 1;
                }
            }
        }
        count
    }

    fn step(&mut self, next: &mut Self, dist: usize, min_taken: i32) -> bool {
        let mut changed = false;
        for y in 0..self.h {
            for x in 0..self.w {
                let new = match self.get(x, y) {
                    Cell::Empty if self.count(x, y, dist) == 0 => {
                        changed = true;
                        Cell::Taken
                    }
                    Cell::Taken if self.count(x, y, dist) >= min_taken => {
                        changed = true;
                        Cell::Empty
                    }
                    cell => cell,
                };

                next.set(x, y, new);
            }
        }
        std::mem::swap(self, next);
        changed
    }

    fn total(&self) -> i32 {
        self.cells.iter().filter(|&&c| c == Cell::Taken).count() as i32
    }

    fn simulate_until_stable(&mut self, dist: usize, min_taken: i32) -> i32 {
        let mut next = self.clone();

        loop {
            let changed = self.step(&mut next, dist, min_taken);
            if !changed {
                return self.total();
            }
        }
    }
}

fn part1(input: &SeatingArea) {
    let mut sa = input.clone();
    println!("{}", sa.simulate_until_stable(1, 4));
}

fn part2(input: &SeatingArea) {
    let mut sa = input.clone();
    println!("{}", sa.simulate_until_stable(0, 5));
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let sa = SeatingArea::from_str(&input);

    part1(&sa);
    part2(&sa);
}
