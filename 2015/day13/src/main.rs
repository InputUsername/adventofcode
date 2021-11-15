use std::collections::{HashMap, HashSet};

fn parse_rule(r: &str) -> (String, String, i32) {
    let mut parts = r.split(' ');

    let name = parts.next().unwrap().to_owned();

    parts.next();

    let change = parts.next().unwrap();
    let delta: i32 = parts.next().unwrap().parse().unwrap();
    let delta = if change == "gain" { delta } else { -delta };

    let other = parts.last().unwrap().trim_end_matches('.').to_owned();

    (name, other, delta)
}

fn parse_input(input: &str) -> (HashSet<String>, HashMap<(String, String), i32>) {
    let mut names = HashSet::new();
    let mut rules = HashMap::new();

    for (name, other, delta) in input.lines().map(|l| parse_rule(l)) {
        names.insert(name.clone());
        rules.insert((name, other), delta);
    }

    (names, rules)
}

fn total_delta(table: &[String], rules: &HashMap<(String, String), i32>) -> i32 {
    let n = table.len();
    let mut sum = 0;
    for i in 0..n {
        let left = if i == 0 { n - 1 } else { i - 1 };
        let left = table[left].clone();
        let right = (i + 1) % n;
        let right = table[right].clone();
        sum += rules.get(&(table[i].clone(), left)).unwrap();
        sum += rules.get(&(table[i].clone(), right)).unwrap();
    }
    sum
}

fn arrange(table: &mut Vec<String>, names: &HashSet<String>, rules: &HashMap<(String, String), i32>) -> i32 {
    if table.len() == names.len() {
        return total_delta(table, rules);
    }

    let mut max_delta = i32::MIN;

    for name in names.iter() {
        if table.contains(name) {
            continue;
        }

        table.push(name.clone());
        let delta = arrange(table, names, rules);
        table.pop();

        if delta > max_delta {
            max_delta = delta;
        }
    }

    max_delta
}

fn main() {
    let (names, rules) = parse_input(&std::fs::read_to_string("input").unwrap());

    part1(&names, &rules);
    part2(names, rules);
}

fn part1(names: &HashSet<String>, rules: &HashMap<(String, String), i32>) {
    let max_delta = arrange(&mut Vec::new(), names, rules);

    println!("{}", max_delta);
}

fn part2(mut names: HashSet<String>, mut rules: HashMap<(String, String), i32>) {
    for name in names.iter() {
        rules.insert(("You".to_owned(), name.clone()), 0);
        rules.insert((name.clone(), "You".to_owned()), 0);
    }
    names.insert("You".to_owned());

    let max_delta = arrange(&mut Vec::new(), &names, &rules);

    println!("{}", max_delta);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_input() -> (HashSet<String>, HashMap<(String, String), i32>) {
        parse_input("Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.")
    }

    #[test]
    fn test_total_delta() {
        
        let (_names, rules) = gen_input();

        let table = ["Alice".to_owned(), "Bob".to_owned(), "Carol".to_owned(), "David".to_owned()];

        assert_eq!(total_delta(&table, &rules), 330);
    }

    #[test]
    fn test_arrange() {
        let (names, rules) = gen_input();

        assert_eq!(arrange(&mut Vec::new(), &names, &rules), 330);
    }
}
