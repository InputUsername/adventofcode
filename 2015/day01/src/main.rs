fn main() {
    let mut input = std::fs::read_to_string("input").unwrap();
    input.pop();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let floor = input
        .chars()
        .fold(0, |acc, c| if c == '(' { acc + 1 } else { acc - 1 });

    println!("{}", floor);
}

fn part2(input: &str) {
    let pos = input
        .chars()
        .scan(0, |acc, c| {
            if c == '(' { *acc += 1 } else { *acc -= 1 };
            Some(*acc)
        })
        .position(|floor| floor == -1)
        .unwrap() + 1;

    println!("{}", pos);
}
