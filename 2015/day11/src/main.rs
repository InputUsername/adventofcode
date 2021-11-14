fn is_valid(password: &[u8]) -> bool {
    let inc = password.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    let safe = password.iter().filter(|&&c| c == b'i' || c == b'o' || c == b'l').count() == 0;
    let mut pairs = Vec::new();
    for (i, pair) in password.windows(2).enumerate() {
        if pair[0] == pair[1] && !pairs.contains(&i) && !pairs.contains(&(i - 1)) {
            pairs.push(i);
        }
    }
    inc && safe && pairs.len() >= 2
}

fn increment(password: &mut [u8]) {
    let mut i = password.len() - 1;
    loop {
        password[i] += 1;
        if password[i] <= b'z' {
            return;
        }
        password[i] = b'a';
        if i == 0 {
            return;
        }
        i -= 1;
    }
}

fn next(password: &mut [u8]) {
    increment(password);
    while !is_valid(password) {
        increment(password);
    }
}

fn main() {
    let input = b"hxbxwxba".to_vec();

    part1(&input);
    part2(&input);
}

fn part1(input: &[u8]) {
    let mut password = input.to_vec();
    next(&mut password);
    println!("{}", String::from_utf8_lossy(&password));
}

fn part2(input: &[u8]) {
    let mut password = input.to_vec();
    next(&mut password);
    next(&mut password);
    println!("{}", String::from_utf8_lossy(&password));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(b"hijklmmn"));
        assert!(!is_valid(b"abbceffg"));
        assert!(!is_valid(b"abbcegjk"));
        
        assert!(!is_valid(b"abcdefgh"));
        assert!(is_valid(b"abcdffaa"));
        
        assert!(!is_valid(b"ghijklmn"));
        assert!(is_valid(b"ghjaabcc"));
    }

    #[test]
    fn test_increment() {
        let mut pass = b"xx".to_vec();
        increment(&mut pass);
        assert_eq!(pass, b"xy");
        increment(&mut pass);
        assert_eq!(pass, b"xz");
        increment(&mut pass);
        assert_eq!(pass, b"ya");
        increment(&mut pass);
        assert_eq!(pass, b"yb");
    }

    #[test]
    fn test_next() {
        let mut pass = b"abcdefgh".to_vec();
        next(&mut pass);
        assert_eq!(pass, b"abcdffaa", "{} != {}", String::from_utf8_lossy(&pass), "abcdffaa");

        //pass = b"ghijklmn".to_vec();
        //next(&mut pass);
        //assert_eq!(pass, b"ghjaabcc", "{} != {}", String::from_utf8_lossy(&pass), "ghjaabcc");
    }
}
