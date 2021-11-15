use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut grid = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert((x as i32, y as i32));
            }
        }
    }

    part1(&grid);
    part2(&grid);
}

fn simulate<F: Fn(&mut HashSet<(i32, i32)>)>(grid: &HashSet<(i32, i32)>, f: F) -> usize {
    let mut curr = grid.clone();
    let mut next = HashSet::with_capacity(100 * 100);
    for _ in 0..100 {
        next.clear();

        for x in 0..100 {
            for y in 0..100 {
                let mut count = 0;
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 { continue; }

                        if curr.contains(&(x + dx, y + dy)) {
                            count += 1;
                        }
                    }
                }
                let live = curr.contains(&(x, y));
                if live && (count == 2 || count == 3) {
                    next.insert((x, y));
                } else if !live && count == 3 {
                    next.insert((x, y));
                }
            }
        }

        f(&mut next);

        std::mem::swap(&mut curr, &mut next);
    }

    curr.len()
}

fn part1(grid: &HashSet<(i32, i32)>) {
    println!("{}", simulate(grid, |_| {}));
}

fn part2(grid: &HashSet<(i32, i32)>) {
    let on = simulate(grid, |grid| {
        grid.insert((0, 0));
        grid.insert((99, 0));
        grid.insert((0, 99));
        grid.insert((99, 99));
    });
    println!("{}", on);
}
