fn main() {
    let input: Vec<(u32, u32, u32)> = std::fs::read_to_string("input").unwrap()
        .lines().map(|l| {
            let mut parts = l.split('x').map(|n| n.parse().unwrap());
            (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    part1(&input);
    part2(&input)
}

fn part1(input: &[(u32, u32, u32)]) {
    let total: u32 = input.iter().map(|&(l, w, h)| {
        let (a, b, c) = (l * w, w * h, h * l);
        2 * a + 2 * b + 2 * c + std::cmp::min(a, std::cmp::min(b, c))
    }).sum();
    
    println!("{}", total);
}

fn part2(input: &[(u32, u32, u32)]) {
    let total: u32 = input.iter().map(|&(l, w, h)| {
        let a = 2 * l + 2 * w;
        let b = 2 * w + 2 * h;
        let c = 2 * h + 2 * l;
        std::cmp::min(a, std::cmp::min(b, c)) + l * w * h
    }).sum();

    println!("{}", total);
}
