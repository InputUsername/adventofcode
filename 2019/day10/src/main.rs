use std::fs;
use std::collections::HashSet;
use std::f32::consts::PI;

type Point = (i32, i32);

fn parse_points(input: &str) -> HashSet<Point> {
    let mut points = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().char_indices() {
            if c == '#' {
                points.insert((x as i32, y as i32));
            }
        }
    }
    points
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let points = parse_points(&input);

    part1(&points);
    part2(&points);
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

fn find_max_visible(points: &HashSet<Point>) -> (Point, usize) {
    let mut point = (-1, -1);
    let mut max_count = 0;

    for &a in points {
        let mut visible = points.clone();
        visible.remove(&a);

        for &b in points {
            if !visible.contains(&b) { continue; }

            let (mut dx, mut dy) = (b.0 - a.0, b.1 - a.1);
            let g = gcd(dx.abs(), dy.abs());
            if g != 0 {
                dx /= g;
                dy /= g;
            }

            let (mut x, mut y) = b;
            while x >= 0 && x <= 19 && y >= 0 && y <= 19 {
                x += dx;
                y += dy;
                visible.remove(&(x, y));
            }
        }

        let count = visible.len();
        if count > max_count {
            point = a;
            max_count = count;
        }
    }

    (point, max_count)
}

fn part1(points: &HashSet<Point>) {
    let (_, c) = find_max_visible(points);

    println!("{}", c);
}

fn direction(a: Point, b: Point) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let mut angle = f32::atan2(dx as f32, -dy as f32);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    angle
}

fn part2(points: &HashSet<Point>) {
    let (a, _) = find_max_visible(points);

    // Find view lines (points at the same angle ordered by distance)
    let mut angles: Vec<(f32, Vec<Point>)> = Vec::new();
    for &b in points.iter() {
        let angle = direction(a, b);
        if let Some((_, line)) = angles.iter_mut().find(|(ang, _)| angle == *ang) {
            line.push(b);
        } else {
            angles.push((angle, vec![b]));
        }
    }

    // Sort view lines by angle
    angles.sort_by(|(a1, _), (a2, _)| a1.partial_cmp(a2).unwrap());

    let mut destroyed = 0;
    'outer: loop {
        for (_, line) in angles.iter_mut() {
            if !line.is_empty() {
                // Keep destroying asteroids until we hit 200
                let last_destroyed = line.remove(0);
                destroyed += 1;
                if destroyed == 200 {
                    println!("{}", 100*last_destroyed.0 + last_destroyed.1);
                    break 'outer;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_visible_1() {
        let points = parse_points(
            "......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####"
        );

        let (p, c) = find_max_visible(&points);

        assert_eq!(p, (5, 8));
        assert_eq!(c, 33);
    }

    #[test]
    fn test_find_max_visible_2() {
        let points = parse_points(
            "#.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###."
        );

        let (p, c) = find_max_visible(&points);

        assert_eq!(p, (1, 2));
        assert_eq!(c, 35);
    }

    #[test]
    fn test_find_max_visible_3() {
        let points = parse_points(
            ".#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#.."
        );

        let (p, c) = find_max_visible(&points);

        assert_eq!(p, (6, 3));
        assert_eq!(c, 41);
    }

    #[test]
    fn test_find_max_visible_4() {
        let points = parse_points(
            ".#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##"
        );

        let (p, c) = find_max_visible(&points);

        assert_eq!(p, (11, 13));
        assert_eq!(c, 210);
    }
}