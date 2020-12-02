use std::fs;

fn main() {
    let input: Vec<(usize, usize, char, String)> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let mut bounds = parts.next().unwrap().split('-')
                .map(|n| n.parse().unwrap());
            let x = bounds.next().unwrap();
            let y = bounds.next().unwrap();
            let c = parts.next().unwrap()[..1].parse().unwrap();
            let pass = parts.next().unwrap().to_string();
            (x, y, c, pass)
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[(usize, usize, char, String)]) {
    let valid = input.iter().filter(|(x, y, c, pass)| {
        (x..=y).contains(&&pass.chars().filter(|pc| pc == c).count())
    }).count();
    println!("{}", valid);
}

fn part2(input: &[(usize, usize, char, String)]) {
    let valid = input.iter().filter(|(x, y, c, pass)| {
        let chars: Vec<char> = pass.chars().collect();
        (&chars[x-1] == c) ^ (&chars[y-1] == c)
    }).count();
    println!("{}", valid);
}
