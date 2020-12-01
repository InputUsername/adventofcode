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
    'outer: for (i, &a) in input.iter().enumerate() {
        for &b in input[i..].iter() {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'outer;
            }
        }
    }
}

fn part2(input: &[i32]) {
    'outer: for (i, &a) in input.iter().enumerate() {
        for (j, &b) in input[i..].iter().enumerate() {
            for &c in input[(i+j)..].iter() {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'outer;
                }
            }
        }
    }
}
