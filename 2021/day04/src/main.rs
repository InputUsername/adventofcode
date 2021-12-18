#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<(u32, bool)>>,
}

impl std::str::FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|l| {
                l.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|n| (n.parse().unwrap(), false))
                    .collect()
            })
            .collect();
        Ok(Self { numbers })
    }
}

impl Board {
    fn mark(&mut self, number: u32) {
        for row in self.numbers.iter_mut() {
            for (n, m) in row.iter_mut() {
                if *n == number {
                    *m = true;
                }
            }
        }
    }

    fn score(&self) -> u32 {
        self.numbers
            .iter()
            .flat_map(|row| row.iter())
            .filter(|(_, m)| !m)
            .map(|(n, _)| *n)
            .sum()
    }

    fn wins(&self) -> Option<u32> {
        let size = self.numbers.len();
        for i in 0..size {
            let mut row_marked = true;
            let mut col_marked = true;
            for j in 0..size {
                row_marked = row_marked && self.numbers[i][j].1;
                col_marked = col_marked && self.numbers[j][i].1;
            }
            if row_marked || col_marked {
                return Some(self.score());
            }
        }
        None
    }
}

fn part1(numbers: &[u32], mut boards: Vec<Board>) {
    'draw: for &n in numbers {
        for board in boards.iter_mut() {
            board.mark(n);
            if let Some(score) = board.wins() {
                println!("{}", n * score);
                break 'draw;
            }
        }
    }
}

fn part2(numbers: &[u32], mut boards: Vec<Board>) {
    let mut winners = Vec::new();
    'draw: for &n in numbers {
        for i in 0..boards.len() {
            if winners.contains(&i) { continue; }
            boards[i].mark(n);
            if let Some(score) = boards[i].wins() {
                if boards.len() - winners.len() == 1 {
                    println!("{}", n * score);
                    break 'draw;
                } else {
                    winners.push(i);
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut parts = input.split("\n\n");

    let numbers: Vec<u32> = parts
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = parts.map(|b| b.parse().unwrap()).collect();

    part1(&numbers, boards.clone());
    part2(&numbers, boards);
}
