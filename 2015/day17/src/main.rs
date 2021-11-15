use std::collections::HashSet;

fn combinations(containers: &[u32], total: u32, containers_used: Option<u32>) -> u32 {
    let mut count = 0;
    let mut unique = HashSet::new();
    for mask in 0..2u32.pow(containers.len() as u32) {
        let mut sum = 0;
        let mut combo = Vec::new();
        for i in 0..containers.len() {
            if (mask >> i) & 1 == 1 {
                sum += containers[i];
                combo.push(i);
            }
        }
        if sum == total && containers_used.map(|u| combo.len() as u32 == u).unwrap_or(true) {
            combo.sort();
            if !unique.contains(&combo) {
                count += 1;
                unique.insert(combo);
            }
        }
    }
    count
}

fn part1(containers: &[u32]) {
    println!("{}", combinations(containers, 150, None));
}

fn min_combinations(containers: &[u32], total: u32) -> u32 {
    let mut min_used = u32::MAX;
    for mask in 0..2u32.pow(containers.len() as u32) {
        let mut sum = 0;
        for i in 0..containers.len() {
            if (mask >> i) & 1 == 1 {
                sum += containers[i];
            }
        }
        let used = mask.count_ones();
        if sum == total && used < min_used {
            min_used = used;
        }
    }
    combinations(containers, total, Some(min_used))
}

fn part2(containers: &[u32]) {
    println!("{}", min_combinations(containers, 150));
}

fn main() {
    let containers: Vec<u32> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&containers);
    part2(&containers);
}
