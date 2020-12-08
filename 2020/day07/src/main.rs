use std::fs;
use std::collections::HashMap;

type BagMap<'a> = HashMap<&'a str, Vec<(i32, &'a str)>>;

fn contains_gold(bag: &str, bags: &BagMap) -> bool {
    let contents = bags.get(bag).unwrap();
    contents.iter().any(|&(_, b)| b == "shiny gold")
    || contents.iter().any(|&(_, b)| contains_gold(b, bags))
}

fn part1(input: &BagMap) {
    let count = input.iter().filter(|(&b, _)| contains_gold(b, input)).count();
    println!("{}", count);
}

fn count_contained_bags(bag: &str, bags: &BagMap) -> i32 {
    bags.get(bag).unwrap()
        .iter()
        .map(|&(n, b)| n + n * count_contained_bags(b, bags))
        .sum()
}

fn part2(input: &BagMap) {
    let count = count_contained_bags("shiny gold", input);
    println!("{}", count);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let bags: BagMap = input.lines()
        .map(|l| {
            let mut parts = l.split(" contain ");
            let container = parts.next().unwrap().trim_end_matches(" bags");
            let contents = parts.next().unwrap();
            let contents: Vec<_> = contents[..contents.len()-1].split(", ")
                .filter_map(|bag| {
                    if bag == "no other bags" {
                        return None;
                    }
                    let bag = bag.trim_end_matches('s').trim_end_matches(" bag");
                    let num_idx = bag.find(' ').unwrap();
                    let num = bag[..num_idx].parse().unwrap();
                    Some((num, &bag[num_idx+1..]))
                })
                .collect();
            (container, contents)
        })
        .collect();

    part1(&bags);
    part2(&bags);
}
