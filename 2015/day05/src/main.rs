fn is_vowel(c: u8) -> bool {
    match c {
        b'a' | b'e' | b'i' | b'o' | b'u' => true,
        _ => false,
    }
}

fn is_nice_1(s: &str) -> bool {
    let bytes = s.as_bytes().to_vec();
    let mut vc = if is_vowel(bytes[0]) { 1 } else { 0 };
    let mut twice = false;
    for window in bytes.windows(2) {
        if is_vowel(window[1]) {
            vc += 1;
        }
        if window[0] == window[1] {
            twice = true;
        }
        if window == b"ab" || window == b"cd" || window == b"pq" || window == b"xy" {
            return false;
        }
    }

    vc >= 3 && twice
}

fn is_nice_2(s: &str) -> bool {
    let bytes = s.as_bytes().to_vec();
    let mut pair = false;
    let mut split_pair = false;

    for (i, window) in bytes.windows(3).enumerate() {
        for pair2 in bytes[i + 2..].windows(2) {
            if window[0] == pair2[0] && window[1] == pair2[1] {
                pair = true;
            }
        }
        if window[0] == window[2] && window[0] != window[1] {
            split_pair = true;
        }
    }

    pair && split_pair
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[String]) {
    let count = input.iter().filter(|s| is_nice_1(s)).count();
    println!("{}", count);
}

fn part2(input: &[String]) {
    let count = input.iter().filter(|s| is_nice_2(s)).count();
    println!("{}", count);
}
