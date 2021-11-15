use std::collections::HashMap;

fn parse_sample() -> HashMap<String, u32> {
    let s = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";
    s.lines()
        .map(|l| {
            let mut parts = l.split(": ");
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn parse_input() -> Vec<HashMap<String, u32>> {
    let input = std::fs::read_to_string("input").unwrap();
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            parts.next();
            parts.next();

            let mut properties = HashMap::new();
            while let Some(prop) = parts.next() {
                let n = parts.next().unwrap().trim_end_matches(',').parse().unwrap();
                properties.insert(prop.trim_end_matches(':').to_owned(), n);
            }
            properties
        })
        .collect()
}

fn part1(sample: &HashMap<String, u32>, input: &[HashMap<String, u32>]) {
    let mut max_count = 0;
    let mut max = usize::MAX;

    for (i, properties) in input.iter().enumerate() {
        let count = properties
            .iter()
            .filter(|&(p, n)| sample.get(p).unwrap() == n)
            .count();

        if count > max_count {
            max_count = count;
            max = i;
        }
    }

    println!("{}", max + 1);
}

fn part2(sample: &HashMap<String, u32>, input: &[HashMap<String, u32>]) {
    let mut max_count = 0;
    let mut max = usize::MAX;

    for (i, properties) in input.iter().enumerate() {
        let count = properties.iter().filter(|&(p, n)| {
            let sn = sample.get(p).unwrap();
            if p == "cats" || p == "trees" {
                n > sn
            } else if p == "pomeranians" || p == "goldfish" {
                n < sn
            } else {
                n == sn
            }
        }).count();

        if count > max_count {
            max_count = count;
            max = i;
        }
    }

    println!("{}", max + 1);
}

fn main() {
    let sample = parse_sample();
    let input = parse_input();

    part1(&sample, &input);
    part2(&sample, &input);
}
