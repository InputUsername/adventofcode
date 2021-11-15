struct Reindeer {
    speed: u32,
    time: u32,
    rest: u32,
}

impl std::str::FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        parts.next();
        parts.next();
        parts.next();
        let speed = parts.next().unwrap().parse().unwrap();
        parts.next();
        parts.next();
        let time = parts.next().unwrap().parse().unwrap();
        for _ in 0..6 {
            parts.next();
        }
        let rest = parts.next().unwrap().parse().unwrap();

        Ok(Self {
            speed,
            time,
            rest,
        })
    }
}

fn main() {
    let input: Vec<Reindeer> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

const TIME: u32 = 2503;

fn part1(reindeer: &[Reindeer]) {
    let mut max_dist = 0;
    for r in reindeer {
        let period = r.time + r.rest;
        let first_dist = (TIME / period) * r.time * r.speed;
        let time_left = TIME - (TIME / period) * period;
        let dist = first_dist + u32::min(time_left, r.time) * r.speed;
        if dist > max_dist {
            max_dist = dist;
        }
    }
    println!("{}", max_dist);
}

fn part2(reindeer: &[Reindeer]) {
    let mut distances = vec![0; reindeer.len()];
    let mut points = vec![0; reindeer.len()];
    for t in 0..TIME {
        for (i, r) in reindeer.iter().enumerate() {
            let period = r.time + r.rest;
            let start = (t / period) * period;
            let end = (t / period) * period + r.time;
            if t >= start && t < end {
                distances[i] += r.speed;
            }
        }
        let max = distances.iter().max().unwrap();
        for (d, p) in distances.iter().zip(points.iter_mut()) {
            if d == max {
                *p += 1;
            }
        }
    }

    println!("{}", points.iter().max().unwrap());
}
