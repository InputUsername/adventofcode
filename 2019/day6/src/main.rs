use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Object {
    name: String,
    parent: Option<String>,
}

impl Object {
    fn new(name: &str) -> Self {
        Object {
            name: name.to_owned(),
            parent: None,
        }
    }
}

fn parse_orbits(input: &str) -> HashMap<String, Object> {
    let mut objects = HashMap::new();
    for rel in input.lines() {
        let mut pair = rel.split(')');
        let name_a = pair.next().unwrap().to_owned();
        let name_b = pair.next().unwrap().to_owned();

        objects.entry(name_a.clone()).or_insert(Object::new(&name_a));

        let b = objects.entry(name_b.clone()).or_insert(Object::new(&name_b));
        b.parent.replace(name_a.clone());
    }
    objects
}

fn main() {
    let objects = parse_orbits(&fs::read_to_string("input").unwrap());

    part1(&objects);
    part2(&objects);
}

fn part1(objects: &HashMap<String, Object>) {
    let mut count = 0;
    for object in objects.values() {
        let mut current = object;
        while let Some(ref parent_name) = current.parent {
            count += 1;
            current = objects.get(parent_name).unwrap();
        }
    }
    println!("{}", count);
}

fn part2(objects: &HashMap<String, Object>) {
    // Find trail from santa to COM
    let mut santa_parents = Vec::new();
    let mut santa = objects.get(&"SAN".to_owned()).unwrap();
    while let Some(ref parent_name) = santa.parent {
        santa_parents.push(parent_name.clone());
        santa = objects.get(parent_name).unwrap();
    }

    // Move towards COM until a common parent with santa is found
    let mut you = objects.get(&"YOU".to_owned()).unwrap();
    let mut transfers = 0;
    while let Some(ref parent_name) = you.parent {
        let common_parent = santa_parents
            .iter()
            .enumerate()
            .find(|(_, name)| name == &parent_name);

        if let Some((i, _)) = common_parent {
            transfers += i;
            break;
        }

        transfers += 1;
        you = objects.get(parent_name).unwrap();
    }

    println!("{}", transfers);
}