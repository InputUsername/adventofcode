use std::collections::HashMap;

type Line = (i32, i32, i32, i32);

fn count_overlaps(lines: &[Line], diagonals: bool) -> usize {
    let mut counts = HashMap::new();
    for &(x0, y0, x1, y1) in lines {
        if x0 == x1 {
            for y in i32::min(y0, y1)..=i32::max(y0, y1) {
                *counts.entry((x0, y)).or_insert(0) += 1;
            }
        } else if y0 == y1 {
            for x in i32::min(x0, x1)..=i32::max(x0, x1) {
                *counts.entry((x, y0)).or_insert(0) += 1;
            }
        } else if diagonals {
            let dx = if x0 > x1 { -1 } else { 1 };
            let dy = if y0 > y1 { -1 } else { 1 };
            let mut x = x0;
            let mut y = y0;
            while x != x1 + dx && y != y1 + dy {
                *counts.entry((x, y)).or_insert(0) += 1;
                x += dx;
                y += dy;
            }
        }
    }
    counts.iter().filter(|(_, c)| **c >= 2).count()
}

fn part1(lines: &[Line]) {
    println!("{}", count_overlaps(lines, false));
}

fn part2(lines: &[Line]) {
    println!("{}", count_overlaps(lines, true));
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let lines: Vec<Line> = input
        .lines()
        .map(|l| {
            let mut coords = l
                .split(" -> ")
                .flat_map(|c| c.split(',').map(|n| n.parse().unwrap()));
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    part1(&lines);
    part2(&lines);
}
