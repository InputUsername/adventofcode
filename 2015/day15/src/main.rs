type Ingredient = [i64; 5];

fn parse_ingredient(s: &str) -> Ingredient {
    let mut parts = s.split(' ');

    parts.next();

    let mut properties = [0; 5];
    for p in properties.iter_mut() {
        parts.next();
        *p = parts.next().unwrap().trim_end_matches(',').parse().unwrap();
    }

    properties
}

fn parse_input(s: &str) -> Vec<Ingredient> {
    s.lines().map(|l| parse_ingredient(l)).collect()
}

fn part1(ingredients: &[Ingredient]) {
    let mut max_score = 0;

    for n0 in 1..=100 {
        for n1 in 1..=100 {
            for n2 in 1..=100 {
                for n3 in 1..=100 {
                    if n0 + n1 + n2 + n3 != 100 {
                        continue;
                    }

                    let mut scores = [0; 4];
                    for i in 0..4 {
                        scores[i] = std::cmp::max(0, ingredients[0][i] * n0 + ingredients[1][i] * n1 + ingredients[2][i] * n2 + ingredients[3][i] * n3);
                    }

                    let score = scores.iter().product();
                    if score > max_score {
                        max_score = score;
                    }
                }
            }
        }
    }

    println!("{}", max_score);
}

fn part2(ingredients: &[Ingredient]) {
    let mut max_score = 0;

    for n0 in 1..=100 {
        for n1 in 1..=100 {
            for n2 in 1..=100 {
                for n3 in 1..=100 {
                    if n0 + n1 + n2 + n3 != 100 { continue; }

                    let mut scores = [0; 5];
                    for i in 0..5 {
                        scores[i] = std::cmp::max(0, ingredients[0][i] * n0 + ingredients[1][i] * n1 + ingredients[2][i] * n2 + ingredients[3][i] * n3);
                    }

                    if scores[4] != 500 {
                        continue;
                    }

                    let score = scores.iter().take(4).product();
                    if score > max_score {
                        max_score = score;
                    }
                }
            }
        }
    }

    println!("{}", max_score);
}

fn main() {
    let input = parse_input(&std::fs::read_to_string("input").unwrap());

    part1(&input);
    part2(&input);
}
