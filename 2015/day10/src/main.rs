fn main() {
    let input: Vec<u32> = b"1321131112".into_iter().map(|d| (d - b'0') as u32).collect();

    part1(input.clone());
    part2(input);
}

fn look_and_say(input: &[u32]) -> Vec<u32> {
    let mut seq = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let d = input[i];
        let mut count = 1;
        i += 1;
        while i < input.len() && input[i] == d {
            count += 1;
            i += 1;
        }
        seq.push(count);
        seq.push(d);
    }
    seq
}

fn part1(mut input: Vec<u32>) {
    for _ in 0..40 {
        input = look_and_say(&input);
    }

    println!("{}", input.len());
}

fn part2(mut input: Vec<u32>) {
    for _ in 0..50 {
        input = look_and_say(&input);
    }

    println!("{}", input.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(&[1]), [1, 1]);
        assert_eq!(look_and_say(&[1, 1]), [2, 1]);
        assert_eq!(look_and_say(&[2, 1]), [1, 2, 1, 1]);
        assert_eq!(look_and_say(&[1, 2, 1, 1]), [1, 1, 1, 2, 2, 1]);
        assert_eq!(look_and_say(&[1, 1, 1, 2, 2, 1]), [3, 1, 2, 2, 1, 1]);
    }
}
