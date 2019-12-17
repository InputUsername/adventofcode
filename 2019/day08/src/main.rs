use std::fs;

const W: usize = 25;
const H: usize = 6;

fn parse_input(input: &str, w: usize, h: usize) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    for (i, d) in input.trim().char_indices() {
        if i % (w * h) == 0 {
            output.push(Vec::new());
        }
        let d = match d {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };
        output.last_mut().unwrap().push(d);
    }
    output
}

fn main() {
    let input = parse_input(&fs::read_to_string("input").unwrap(), W, H);

    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<u8>]) {
    let n = input.iter()
        .min_by_key(|layer| layer.iter().filter(|&&d| d == 0).count())
        .map(|layer| layer.iter().filter(|&&d| d == 1).count() * layer.iter().filter(|&&d| d == 2).count())
        .unwrap();

    println!("{}", n);
}

fn part2(input: &[Vec<u8>]) {
    let mut output = vec![2; W * H];

    for layer in input {
        for (i, &d) in layer.iter().enumerate() {
            if output[i] == 2 {
                output[i] = d;
            }
        }
    }

    for row in output.chunks(W) {
        for &d in row {
            print!("{}", if d == 0 { " " } else { "#" });
        }
        println!();
    }
}