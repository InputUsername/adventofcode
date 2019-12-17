use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|n| n.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[i32]) {
    let fuel: i32 = input
        .iter()
        .map(|mass| mass / 3 - 2)
        .sum();

    println!("{}", fuel);
}

fn part2(input: &[i32]) {
    let fuel: i32 = input
        .iter()
        .map(|mass| {
            let mut next = mass / 3 - 2;
            let mut total = 0;
            while next > 0 {
                total += next;
                next = next / 3 - 2;
            }
            total
        })
        .sum();

    println!("{}", fuel);
}