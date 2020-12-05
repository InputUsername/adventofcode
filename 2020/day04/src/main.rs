use std::fs;
use std::collections::HashMap;

fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    REQUIRED_FIELDS.iter().all(|&field| passport.contains_key(field))
}

fn part1(input: &[HashMap<&str, &str>]) {
    let count = input.iter()
        .filter(|&passport| has_required_fields(passport))
        .count();
    println!("{}", count);
}

fn validate_fields(passport: &HashMap<&str, &str>) -> bool {
    if !has_required_fields(passport) {
        return false;
    }

    // check byr, iyr, eyr

    const YEAR_FIELDS: [(&str, i32, i32); 3] = [
        ("byr", 1920, 2002),
        ("iyr", 2010, 2020),
        ("eyr", 2020, 2030),
    ];
    let invalid = YEAR_FIELDS.iter().any(|&(field, x, y)| {
        passport.get(field).unwrap().parse()
            .map(|value: i32| value < x || value > y)
            .unwrap_or(false)
    });
    if invalid {
        return false;
    }

    // check hgt

    let &hgt = passport.get("hgt").unwrap();
    let suffix = hgt.get(hgt.len()-2..);
    let value = suffix.and_then(|s| hgt.trim_end_matches(s).parse::<i32>().ok());
    match (suffix, value) {
        (Some("cm"), Some(n)) => if n < 150 || n > 193 { return false; }
        (Some("in"), Some(n)) => if n < 59 || n > 76 { return false; }
        _ => return false,
    }

    // check hcl

    let &hcl = passport.get("hcl").unwrap();
    if &hcl[..1] != "#" || hcl.len() != 7 || i64::from_str_radix(&hcl[1..], 16).is_err() {
        return false;
    }

    // check ecl

    const VALID_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let invalid = VALID_COLORS.iter().all(|c| passport.get("ecl").unwrap() != c);
    if invalid {
        return false;
    }

    // check pid

    let &pid = passport.get("pid").unwrap();
    if pid.len() != 9 || pid.parse::<i64>().is_err() {
        return false;
    }

    true
}

fn part2(input: &[HashMap<&str, &str>]) {
    let count = input.iter()
        .filter(|&passport| {
            //println!("{:?}", passport);
            validate_fields(passport)
        })
        .count();
    println!("{}", count);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let passports: Vec<HashMap<_, _>> = input
        .split("\n\n")
        .map(|passport| {
            passport.split_whitespace()
                .map(|field| {
                    let mut parts = field.split(':');
                    (parts.next().unwrap(), parts.next().unwrap())
                })
                .collect()
        })
        .collect();

    part1(&passports);
    part2(&passports);
}
