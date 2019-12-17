use std::fs;

struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    steps: i32,
}

struct Intersection {
    x: i32,
    y: i32,
    steps: i32,
}

impl Segment {
    /// Normalize a segment so it always points right or down.
    fn normalize(&self) -> Segment {
        Segment {
            x1: i32::min(self.x1, self.x2),
            y1: i32::min(self.y1, self.y2),
            x2: i32::max(self.x1, self.x2),
            y2: i32::max(self.y1, self.y2),
            steps: -1,
        }
    }

    /// Find the intersection point with another segment if it exists.
    /// Also returns the number of combined steps from the starting point.
    fn intersect(&self, other: &Segment) -> Option<Intersection> {
        // Central point is not an intersection
        if self.x1 == 0 && self.y1 == 0 && other.x1 == 0 && other.y1 == 0 {
            return None;
        }

        // Self vertical, other horizontal
        if self.x1 == self.x2 && other.y1 == other.y2 {
            let self_norm = self.normalize();
            let other_norm = other.normalize();
            if other_norm.x1 <= self_norm.x1 && other_norm.x2 >= self_norm.x2
                && self_norm.y1 <= other_norm.y1 && self_norm.y2 >= other_norm.y2
            {
                let x = self_norm.x1;
                let y = other_norm.y1;
                let steps = self.steps + other.steps - (y - self.y2).abs() - (x - other.x2).abs();

                return Some(Intersection { x, y, steps, });
            }
        // Self horizontal, other vertical
        } else if self.y1 == self.y2 && other.x1 == other.x2 {
            return other.intersect(self);
        }

        None
    }
}

fn main() {
    let wires: Vec<Vec<Segment>> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| parse_wire(line))
        .collect();

    part1(&wires[0], &wires[1]);
    part2(&wires[0], &wires[1]);
}

fn parse_wire(line: &str) -> Vec<Segment> {
    let mut wire = Vec::new();

    line.split(',')
        .fold((0, 0, 0), |(x, y, steps), movement| {
            let n: i32 = movement[1..].parse().unwrap();
            let next = match &movement[..1] {
                "U" => (x, y + n),
                "D" => (x, y - n),
                "L" => (x - n, y),
                "R" => (x + n, y),
                _ => unreachable!(),
            };
            wire.push(Segment {
                x1: x,
                y1: y,
                x2: next.0,
                y2: next.1,
                steps: steps + n,
            });
            (next.0, next.1, steps + n)
        });

    wire
}

/// Part 1: find the intersection closest to the central point.
fn part1(wire_1: &[Segment], wire_2: &[Segment]) {
    let mut min_dist = i32::max_value();
    for segment_1 in wire_1 {
        for segment_2 in wire_2 {
            if let Some(Intersection { x, y, steps: _ }) = segment_1.intersect(segment_2) {
                let dist = x + y;
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }
    println!("{}", min_dist);
}

/// Part 2: find the intersection with the least amount of combined steps from the central point.
fn part2(wire_1: &[Segment], wire_2: &[Segment]) {
    let mut min_steps = i32::max_value();
    for segment_1 in wire_1 {
        for segment_2 in wire_2 {
            if let Some(intersection) = segment_1.intersect(segment_2) {
                if intersection.steps < min_steps {
                    min_steps = intersection.steps;
                }
            }
        }
    }
    println!("{}", min_steps);
}