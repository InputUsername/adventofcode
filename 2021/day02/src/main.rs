enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_command(c: &str) -> Command {
    let mut parts = c.split(' ');
    let cmd = parts.next().unwrap();
    let n = parts.next().unwrap().parse().unwrap();
    if cmd == "forward" {
        Command::Forward(n)
    } else if cmd == "up" {
        Command::Up(n)
    } else {
        Command::Down(n)
    }
}

fn part1(input: &[Command]) {
    let (hor, depth) = input.iter().fold((0, 0), |(hor, depth), cmd| match cmd {
        Command::Forward(n) => (hor + n, depth),
        Command::Up(n) => (hor, depth - n),
        Command::Down(n) => (hor, depth + n),
    });
    println!("{}", hor * depth);
}

fn part2(input: &[Command]) {
    let (hor, depth, _) = input
        .iter()
        .fold((0, 0, 0), |(hor, depth, aim), cmd| match cmd {
            Command::Forward(n) => (hor + n, depth + n * aim, aim),
            Command::Up(n) => (hor, depth, aim - n),
            Command::Down(n) => (hor, depth, aim + n),
        });
    println!("{}", hor * depth);
}

fn main() {
    let input: Vec<Command> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| parse_command(l))
        .collect();

    part1(&input);
    part2(&input);
}
