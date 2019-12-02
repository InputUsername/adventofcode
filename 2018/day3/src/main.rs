use std::fs;
use std::collections::HashSet;

struct Claim {
    n: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

struct FabricSquare {
    claims: Vec<usize>,
    count: usize,
}

fn main() {
    let input: Vec<Claim> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| parse_claim(l))
        .collect();

    part1(&input);
    part2(&input);
}

fn parse_claim(line: &str) -> Claim {
    let nums: Vec<_> = line.trim()
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|part| part.parse().ok())
        .collect();

    Claim {
        n: nums[0],
        x: nums[1],
        y: nums[2],
        w: nums[3],
        h: nums[4],
    }
}

fn part1(input: &[Claim]) {
    let mut fabric = vec![0; 1000*1000];

    for claim in input {
        for x in claim.x..claim.x+claim.w {
            for y in claim.y..claim.y+claim.h {
                fabric[1000*y + x] += 1;
            }
        }
    }

    let count = fabric.iter()
        .filter(|&&n| n >= 2)
        .count();

    println!("{}", count);
}

fn part2(input: &[Claim]) {
    let mut fabric = Vec::with_capacity(1000*1000);
    for _ in 0..1000*1000 {
        fabric.push(FabricSquare {
            claims: Vec::new(),
            count: 0,
        });
    }
    let mut intact_claims: HashSet<usize> = (1..=input.len()).collect();

    for claim in input {
        for x in claim.x..claim.x+claim.w {
            for y in claim.y..claim.y+claim.h {
                let idx = 1000*y + x;

                fabric[idx].count += 1;
                fabric[idx].claims.push(claim.n);

                if fabric[idx].claims.len() > 1 {
                    for c in fabric[idx].claims.iter() {
                        intact_claims.remove(c);
                    }
                }
            }
        }
    }

    let claim = intact_claims.iter().next().unwrap();

    println!("{}", claim);
}