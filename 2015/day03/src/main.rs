use std::collections::HashSet;

fn main() {
    let mut input = std::fs::read_to_string("input").unwrap();
    input.pop();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut map = HashSet::new();
    map.insert((0, 0));
    let (mut x, mut y) = (0, 0);

    for c in input.chars() {
        match c {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => unreachable!(),
        }

        map.insert((x, y));
    }

    println!("{}", map.len());
}
fn part2(input: &str) {
    let mut map = HashSet::new();
    map.insert((0, 0));
    let (mut x, mut y) = ([0, 0], [0, 0]);
    let mut who = 0;

    for c in input.chars() {
        match c {
            '^' => y[who] -= 1,
            'v' => y[who] += 1,
            '<' => x[who] -= 1,
            '>' => x[who] += 1,
            _ => unreachable!(),
        }

        map.insert((x[who], y[who]));

        who = 1 - who;
    }

    println!("{}", map.len());
}
