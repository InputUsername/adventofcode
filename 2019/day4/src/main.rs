use std::fs;

fn main() {
    let numbers = fs::read_to_string("input").unwrap();

    let a = numbers[..6].parse().unwrap();
    let b = numbers[7..].parse().unwrap();

    part1(a, b);
    part2(a, b);
}

fn is_valid(mut n: u32) -> bool {
    let mut double = false;
    while n > 0 {
        let d = n % 10;
        if n >= 10 {
            let next = (n / 10) % 10;
            if next == d {
                double = true;
            }
            if next > d {
                return false;
            }
        }
        n /= 10;
    }
    double
}

fn is_valid_ext(mut n: u32) -> bool {
    let mut double = false;
    let mut d = n % 10;
    let mut group_count = 1;
    while n > 0 {
        n /= 10;
        let next = n % 10;
        if next > d {
            return false;
        }
        if next != d {
            if group_count == 2 {
                double = true;
            }
            group_count = 1;
        } else {
            group_count += 1;
        }
        d = next;
    }
    double
}

fn part1(a: u32, b: u32) {
    let count = (a..=b).filter(|&n| is_valid(n)).count();
    println!("{}", count);
}

fn part2(a: u32, b: u32) {
    let count = (a..=b).filter(|&n| is_valid_ext(n)).count();
    println!("{}", count);
}