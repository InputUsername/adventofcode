use serde_json::Value;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input: Value = serde_json::from_str(&input).unwrap();

    part1(input.clone());
    part2(input);
}

fn sum(val: Value) -> Option<i64> {
    match val {
        Value::Number(n) => n.as_i64(),
        Value::Array(a) => Some(a.into_iter().filter_map(sum).sum()),
        Value::Object(o) => Some(o.into_iter().filter_map(|(_, v)| sum(v)).sum()),
        _ => None,
    }
}

fn part1(val: Value) {
    println!("{}", sum(val).unwrap());
}

fn sum_without_red(val: Value) -> Option<i64> {
    match val {
        Value::Number(n) => n.as_i64(),
        Value::Array(a) => Some(a.into_iter().filter_map(sum_without_red).sum()),
        Value::Object(o) => {
            if o.values().any(|v| v.is_string() && v.as_str().unwrap() == "red") {
                None
            } else {
                Some(o.into_iter().filter_map(|(_, v)| sum_without_red(v)).sum())
            }
        }
        _ => None,
    }
}

fn part2(val: Value) {
    println!("{}", sum_without_red(val).unwrap());
}
