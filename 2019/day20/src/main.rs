use std::fs;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::{Ord, Ordering};

mod input;

use input::parse_input;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Floor,
    Wall,
    Start,
    End,
    Portal(usize, usize),

    Letter(char),
}

fn print_map(map: &Vec<Vec<Tile>>, route: Option<&HashSet<(usize, usize)>>) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if route.is_some() && route.unwrap().contains(&(x, y)) {
                print!("█");
            } else {
                print!("{}", match tile {
                    Tile::Empty => ' ',
                    Tile::Floor => ' ',
                    Tile::Wall => '▒',
                    Tile::Start => '$',
                    Tile::End => '%',
                    Tile::Portal(_, _) => '@',
                    Tile::Letter(c) => *c,
                });
            }
        }
        println!();
    }
}

type Point = (usize, usize, usize);

#[derive(Eq, PartialEq)]
struct Cell {
    pos: Point,
    d: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> Ordering {
        other.d.cmp(&self.d)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn adjacent_cells(pos: Point, map: &Vec<Vec<Tile>>, recurse: bool) -> Vec<Point> {
    let (w, h) = (map[0].len(), map.len());
    let (x, y, z) = pos;
    let (xi, yi) = (x as i32, y as i32);

    let mut adj = Vec::new();
    for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (cx, cy) = ((xi + dx) as usize, (yi + dy) as usize);
        match map[cy][cx] {
            Tile::Floor | Tile::Portal(_, _)
                | Tile::Start | Tile::End => adj.push((cx, cy, z)),
            Tile::Letter(_) => {
                if let Tile::Portal(target_x, target_y) = map[y][x] {
                    if !recurse {
                        adj.push((target_x, target_y, z));
                        continue;
                    }

                    // At z=0, only the inner portals work
                    let is_outer = x == 2 || y == 2 || x == w-3 || y == h-3;
                    if z != 0 {
                        let target_z = if is_outer { z - 1 } else { z + 1 };
                        adj.push((target_x, target_y, target_z));
                    } else if !is_outer {
                        adj.push((target_x, target_y, z + 1));
                    }
                }
            }
            _ => {}
        }
    }
    adj
}

fn dijkstra(map: &Vec<Vec<Tile>>, start: Point, end: Point, recurse: bool) -> Option<usize> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    dist.insert(start, 0);
    queue.push(Cell { pos: start, d: 0 });

    while let Some(Cell { pos: cur_pos, d: cur_d }) = queue.pop() {
        if cur_pos == end { return Some(cur_d); }

        let old_d = *dist.entry(cur_pos).or_insert(usize::max_value());
        if cur_d > old_d { continue; }

        let adj = adjacent_cells(cur_pos, map, recurse);
        for next_pos in adj.into_iter() {
            let next = Cell { pos: next_pos, d: cur_d + 1 };
            let next_d = dist.entry(next_pos).or_insert(usize::max_value());
            if next.d < *next_d {
                *next_d = next.d;
                queue.push(next);
            }
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (map, start, end) = parse_input(&input);
    let start = (start.0, start.1, 0);
    let end = (end.0, end.1, 0);

    part1(&map, start, end);
    part2(&map, start, end);
}

fn part1(map: &Vec<Vec<Tile>>, start: Point, end: Point) {
    let dist = dijkstra(&map, start, end, false);
    println!("{}", dist.unwrap());
}

fn part2(map: &Vec<Vec<Tile>>, start: Point, end: Point) {
    let dist = dijkstra(&map, start, end, true);
    println!("{}", dist.unwrap());
}