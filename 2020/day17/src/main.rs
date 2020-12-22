use std::fs;
use std::collections::HashSet;

type Grid = HashSet<[i64; 4]>;

fn parse(s: &str) -> Grid {
    s.lines().enumerate().flat_map(|(y, l)| {
        l.char_indices().filter_map(move |(x, c)| match c {
            '#' => Some([x as i64, y as i64, 0, 0]),
            _ => None,
        })
    }).collect()
}

fn active_neighbors(x: i64, y: i64, z: i64, w: i64, grid: &Grid) -> usize {
    let mut count = 0;
    for dw in -1..=1 {
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }

                    if grid.contains(&[x + dx, y + dy, z + dz, w + dw]) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn step(dim: usize, curr: &mut Grid, next: &mut Grid) {
    let mut min = [0; 4];
    let mut max = [0; 4];
    for coords in curr.iter() {
        for i in 0..4 {
            min[i] = min[i].min(coords[i]);
            max[i] = max[i].max(coords[i]);
        }
    }

    let w_range = if dim == 3 { 0..=0 } else { min[3]-1..=max[3]+1 };

    for w in w_range {
        for z in min[2]-1..=max[2]+1 {
            for y in min[1]-1..=max[1]+1 {
                for x in min[0]-1..=max[0]+1 {
                    let is_active = curr.contains(&[x, y, z, w]);
                    let n = active_neighbors(x, y, z, w, curr);
                    if is_active && (n == 2 || n == 3) {
                        next.insert([x, y, z, w]);
                    } else if !is_active && n == 3 {
                        next.insert([x, y, z, w]);
                    }
                }
            }
        }
    }

    curr.clear();
    std::mem::swap(curr, next);
}

fn simulate(dim: usize, grid: &Grid) -> usize {
    let mut curr = grid.clone();
    let mut next = HashSet::new();

    for _ in 0..6 {
        step(dim, &mut curr, &mut next);
    }

    curr.len()
}

fn part1(grid: &Grid) {
    println!("{}", simulate(3, grid));
}

fn part2(grid: &Grid) {
    println!("{}", simulate(4, grid));
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = parse(&input);

    part1(&input);
    part2(&input);
}
