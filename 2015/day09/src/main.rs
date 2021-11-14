use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> (HashSet<String>, HashMap<(String, String), u32>) {
    let mut locations = HashSet::new();
    let mut table = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(' ').map(|s| s.to_owned());
        let (from, _, to, _, dist) = (parts.next().unwrap(), parts.next(), parts.next().unwrap(), parts.next(), parts.next().unwrap().parse().unwrap());
        locations.insert(from.clone());
        locations.insert(to.clone());
        table.insert((from.clone(), to.clone()), dist);
        table.insert((to, from), dist);
    }
    (locations, table)
}

struct Node<'a> {
    loc: String,
    visited: HashSet<&'a String>,
    dist: u32,
}

fn path(locations: &HashSet<String>, table: &HashMap<(String, String), u32>, init: u32, cmp: impl Fn(u32, u32) -> u32) -> u32 {
    let mut frontier = VecDeque::new();
    for loc in locations.iter() {
        frontier.push_back(Node {
            loc: loc.clone(),
            visited: vec![loc].into_iter().collect(),
            dist: 0,
        });
    }

    let mut min_dist = init;

    while let Some(node) = frontier.pop_front() {
        if node.visited.len() == locations.len() {
            min_dist = cmp(min_dist, node.dist);
            continue;
        }

        for ((from, to), dist) in table.iter() {
            if from != &node.loc || node.visited.contains(to) {
                continue;
            }

            let mut visited = node.visited.clone();
            visited.insert(to);

            frontier.push_back(Node {
                loc: to.clone(),
                visited,
                dist: node.dist + dist,
            });
        }
    }

    min_dist
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (locations, table) = parse_input(&input);

    part1(&locations, &table);
    part2(&locations, &table);
}

fn part1(locations: &HashSet<String>, table: &HashMap<(String, String), u32>) {
    let dist = path(locations, table, u32::MAX, std::cmp::min);
    println!("{}", dist);
}

fn part2(locations: &HashSet<String>, table: &HashMap<(String, String), u32>) {
    let dist = path(locations, table, 0, std::cmp::max);
    println!("{}", dist);
}
