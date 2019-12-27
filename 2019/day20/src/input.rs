use std::collections::HashMap;

use crate::Tile;

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

pub(crate) fn parse_input(input: &str) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
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