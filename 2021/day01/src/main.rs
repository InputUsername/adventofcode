fn main() {
    let input: Vec<u32> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[u32]) {
    let count = input.windows(2).filter(|w| w[1] > w[0]).count();
    println!("{}", count);
}

fn part2(input: &[u32]) {
    let windows: Vec<&[u32]> = input.windows(3).collect();
    let count = windows
        .windows(2)
        .filter(|ws| ws[1].iter().sum::<u32>() > ws[0].iter().sum())
        .count();
    println!("{}", count);
}
