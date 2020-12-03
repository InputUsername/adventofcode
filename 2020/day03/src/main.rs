use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree,
}

struct Map {
    map: Vec<Tile>,
    w: usize,
    h: usize,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = s.find('\n')
            .ok_or_else(|| "Could not determine input width".to_string())?;
        let map: Vec<Tile> = s.lines()
            .flat_map(|l| l.trim().chars())
            .map(|c| match c {
                '.' => Ok(Tile::Open),
                '#' => Ok(Tile::Tree),
                _ => Err(format!("Unknown character {}", c)),
            })
            .collect::<Result<_, _>>()?;
        let h = map.len() / w;

        Ok(Self {
            map,
            w,
            h,
        })
    }
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Tile {
        self.map[self.w * y + (x % self.w)]
    }

    fn count_trees_by(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut count = 0;
        for y in (0..self.h).step_by(dy) {
            if self.at(x, y) == Tile::Tree {
                count += 1;
            }
            x += dx;
        }

        count
    }
}

fn part1(input: &Map) {
    println!("{}", input.count_trees_by(3, 1));
}

fn part2(input: &Map) {
    let count: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|&(dx, dy)| input.count_trees_by(dx, dy))
        .product();
    println!("{}", count);
}

fn main() {
    let map: Map = fs::read_to_string("input").unwrap().parse().unwrap();

    part1(&map);
    part2(&map);
}
