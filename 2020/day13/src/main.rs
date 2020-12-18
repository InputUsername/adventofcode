use std::fs;

fn parse_1(s: &str) -> (i32, Vec<i32>) {
    let mut lines = s.lines();
    let t = lines.next().unwrap().parse().unwrap();
    let b = lines.next().unwrap()
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect();
    (t, b)
}

fn part1(input: &str) {
    let (t, b) = parse_1(input);
    let (id, time) = b.iter()
        // find next departure time after start
        .map(|&id| (id, t - (t % id) + id))
        // find bus with closest departure time
        .min_by_key(|(_, departure)| *departure)
        .unwrap();

    let wait = time - t;

    println!("{}", id * wait);
}

fn parse_2(s: &str) -> Vec<(i64, i64)> {
    s.lines().skip(1).next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(offset, id)| (id.parse().unwrap(), offset as i64))
        .collect()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> i64 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn solve_crt(buses: &[(i64, i64)]) -> i64 {
    let prod: i64 = buses.iter().map(|(id, _)| *id).product();

    let mut sum = 0;

    for &(id, off) in buses.iter() {
        let p = prod / id;
        sum += off * mod_inv(p, id) * p;
    }

    -(sum % prod - prod)
}

fn part2(input: &str) {
    let b = parse_2(input);

    println!("{}", solve_crt(&b));
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    part1(&input);
    part2(&input);
}
