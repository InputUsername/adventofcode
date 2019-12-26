use std::fs;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::{Ord, Ordering};

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

enum LetterTileType {
    Start,
    End,
    Portal(char, char),
}

fn check_portal_start_end(x: usize, y: usize, map: &Vec<Vec<Tile>>) -> Option<LetterTileType> {
    let (w, h) = (map[0].len(), map.len());
    if x < 2 || y < 2 || x > w-2 || y > h-2 {
        return None;
    }

    if map[y][x] != Tile::Floor {
        return None;
    }

    let (xi, yi) = (x as i32, y as i32);

    for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (x1, y1) = ((xi + dx) as usize, (yi + dy) as usize);
        let (x2, y2) = ((xi + 2*dx) as usize, (yi + 2*dy) as usize);

        match (map[y1][x1], map[y2][x2]) {
            (Tile::Letter('A'), Tile::Letter('A')) => return Some(LetterTileType::Start),
            (Tile::Letter('Z'), Tile::Letter('Z')) => return Some(LetterTileType::End),
            (Tile::Letter(a), Tile::Letter(b)) => {
                if dx == -1 || dy == -1 {
                    return Some(LetterTileType::Portal(b, a));
                } else {
                    return Some(LetterTileType::Portal(a, b));
                }
            }
            _ => {}
        }
    }

    None
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<Tile>> = input.lines().map(|line| {
        line.chars().map(|c| match c {
            ' ' => Tile::Empty,
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            _ if c.is_alphabetic() => Tile::Letter(c),
            _ => Tile::Empty,
        }).collect()
    }).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut portals = HashMap::new();

    let h = map.len();
    let w = map[0].len();
    for y in 0..h {
        for x in 0..w {
            match check_portal_start_end(x, y, &map) {
                Some(LetterTileType::Start) => {
                    map[y][x] = Tile::Start;
                    start = (x, y);
                }
                Some(LetterTileType::End) => {
                    map[y][x] = Tile::End;
                    end = (x, y);
                },
                Some(LetterTileType::Portal(a, b)) => {
                    if let Some((other_x, other_y)) = portals.remove(&(a, b)) {
                        map[y][x] = Tile::Portal(other_x, other_y);
                        map[other_y][other_x] = Tile::Portal(x, y);
                    } else {
                        portals.insert((a, b), (x, y));
                    }
                }
                None => {}
            }
        }
    }

    (map, start, end)
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

#[derive(Eq, PartialEq)]
struct Cell {
    pos: (usize, usize),
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

fn adjacent_cells(x: usize, y: usize, map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let (xi, yi) = (x as i32, y as i32);
    let mut adj = Vec::new();
    for &(dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let (cx, cy) = ((xi + dx) as usize, (yi + dy) as usize);
        match map[cy][cx] {
            Tile::Floor | Tile::Portal(_, _)
                | Tile::Start | Tile::End => adj.push((cx, cy)),
            Tile::Letter(_) => {
                if let Tile::Portal(target_x, target_y) = map[y][x] {
                    adj.push((target_x, target_y));
                }
            }
            _ => {}
        }
    }
    adj
}

fn dijkstra(map: &Vec<Vec<Tile>>, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut dist = HashMap::new();
    let mut queue = BinaryHeap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            dist.insert((x, y), usize::max_value());
        }
    }

    dist.insert(start, 0);
    queue.push(Cell { pos: start, d: 0 });

    while let Some(Cell { pos: cur_pos, d: cur_d }) = queue.pop() {
        if cur_d > *dist.get(&cur_pos).unwrap() { continue; }

        let adj = adjacent_cells(cur_pos.0, cur_pos.1, map);
        for next_pos in adj.into_iter() {
            let next = Cell { pos: next_pos, d: cur_d + 1 };
            if next.d < *dist.get(&next_pos).unwrap() {
                dist.insert(next_pos, next.d);
                queue.push(next);
            }
        }
    }

    dist
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let (map, start, end) = parse_input(&input);

    let dist = dijkstra(&map, start);

    println!("{:?}", end);

    let mut route = HashSet::new();
    let mut pos = end;
    while pos != start {
        let adj = adjacent_cells(pos.0, pos.1, &map);
        pos = adj.into_iter()
            .min_by_key(|pos| dist.get(&pos).unwrap())
            .unwrap();

        if pos != start {
            route.insert(pos);
        }
    }

    print_map(&map, Some(&route));

    println!("{}", dist.get(&end).unwrap());
}