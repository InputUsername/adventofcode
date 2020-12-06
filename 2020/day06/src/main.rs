use std::fs;
use std::collections::HashSet;

fn part1(input: &[Vec<&str>]) {
    let sum: usize = input.iter()
        .map(|group| {
            group.iter().flat_map(|answers| answers.chars()).collect::<HashSet<char>>()
        })
        .map(|answers| answers.len())
        .sum();
    println!("{}", sum);
}

fn part2(input: &[Vec<&str>]) {
    let sum: usize = input.iter()
        .map(|group| {
            let mut answers: Vec<HashSet<char>> = group.iter()
                .map(|answers| answers.chars().collect::<HashSet<char>>())
                .collect();
            let last = answers.pop().unwrap();
            answers.into_iter()
                .fold(last, |acc, curr| {
                    acc.intersection(&curr).copied().collect()
                })
                .len()
        })
        .sum();

    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answers: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|group| group.lines().collect())
        .collect();

    part1(&answers);
    part2(&answers);
}
