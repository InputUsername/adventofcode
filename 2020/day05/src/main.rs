use std::fs;

fn bsp(s: &str, n_lo: i32, n_hi: i32) -> i32 {
    s.chars().fold((n_lo, n_hi), |(lo, hi), c| {
        if c == 'F' || c == 'L' {
            (lo, (lo+hi)/2)
        } else {
            ((lo+hi)/2+1, hi)
        }
    }).0
}

fn iter_ids<'a>(input: &'a [(&str, &str)]) -> impl Iterator<Item=i32> + 'a {
    input.iter()
        .map(|&(row, col)| (bsp(row, 0, 127), bsp(col, 0, 7)))
        .map(|(row, col)| row*8 + col)
}

fn part1(input: &[(&str, &str)]) {
    let max_id = iter_ids(input)
        .max()
        .unwrap();

    println!("{}", max_id);
}

fn part2(input: &[(&str, &str)]) {
    let mut ids: Vec<i32> = iter_ids(input).collect();
    ids.sort();
    let missing = ids.windows(2)
        .find(|&w| w[1] - w[0] == 2)
        .map(|w| w[0] + 1)
        .unwrap();

    println!("{}", missing);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let passes: Vec<(&str, &str)> = input.lines()
        .map(|l| (&l[..7], &l[7..]))
        .collect();

    part1(&passes);
    part2(&passes);
}
