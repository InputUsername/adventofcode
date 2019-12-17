use std::fs;

use intcode::Computer;

fn main() {
    let input: Vec<i64> = fs::read_to_string("input")
        .unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let cpu = Computer::from(&input[..]);

    part1(cpu.clone());
}

fn part1(mut cpu: Computer) {
    let output = cpu.run(&[]);

    let image: Vec<Vec<char>> = output.split(|&n| n == 10)
        .take_while(|row| !row.is_empty())
        .map(|row| row.iter().map(|&n| (n as u8).into()).collect())
        .collect();

    let mut total = 0;
    let h = image.len();
    for y in 1..(h-1) {
        let w = image[y].len();
        for x in 1..(w-1) {
            if image[y][x] == '#' && image[y][x-1] == '#' && image[y][x+1] == '#'
                && image[y-1][x] == '#' && image[y+1][x] == '#'
            {
                total += x * y;
            }
        }
    }

    println!("{}", total);
}