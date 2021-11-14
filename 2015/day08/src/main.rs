fn escape(s: &str) -> Vec<u8> {
    let s = &s[1..s.len()-1];
    let bs = s.as_bytes();
    let mut e = Vec::new();
    let mut i = 0;
    while i < bs.len() {
        let b0 = bs[i];
        if b0 == b'\\' {
            i += 1;
            let b1 = bs[i];
            if b1 == b'x' {
                i += 1;
                e.push(u8::from_str_radix(&s[i..i+2], 16).unwrap());
                i += 2;
            } else {
                e.push(b1);
                i += 1;
            }
        } else {
            e.push(b0);
            i += 1;
        }
    }
    e
}

fn encode(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
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
    let diff: usize = input.iter().map(|s| s.len() - escape(s).len()).sum();
    println!("{}", diff);
}

fn part2(input: &[String]) {
    let diff: usize = input.iter().map(|s| encode(s).len() - s.len()).sum();
    println!("{}", diff);
}
