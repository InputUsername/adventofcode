fn simulate(fish: &[u8], days: u32) -> u64 {
    let mut buckets = vec![0; 9];
    for &days_left in fish {
        buckets[days_left as usize] += 1;
    }

    for _ in 0..days {
        buckets.rotate_left(1);
        buckets[6] += buckets[8];
    }

    buckets.into_iter().sum()
}

fn part1(fish: &[u8]) {
    println!("{}", simulate(fish, 80));
}

fn part2(fish: &[u8]) {
    println!("{}", simulate(fish, 256));
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let fish: Vec<u8> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    part1(&fish);
    part2(&fish);
}
