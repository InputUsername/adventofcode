use std::fs;
use std::collections::HashMap;
use std::ops::RangeInclusive;

type TicketRules = HashMap<String, Vec<RangeInclusive<i64>>>;

#[derive(Debug)]
struct Input {
    rules: TicketRules,
    ticket: Vec<i64>,
    other_tickets: Vec<Vec<i64>>,
}

fn parse(s: &str) -> Input {
    let mut sections = s.split("\n\n");

    let rules = sections.next().unwrap().lines().map(|l| {
        let mut parts = l.split(": ");
        let field = parts.next().unwrap().to_string();
        let ranges = parts.next().unwrap().split(" or ")
            .map(|range| {
                let sep = range.find('-').unwrap();
                let start = range[..sep].parse().unwrap();
                let end = range[sep+1..].parse().unwrap();
                 start ..= end
            })
            .collect();
        (field, ranges)
    })
    .collect();

    let ticket = sections.next().unwrap()
        .lines().skip(1).next().unwrap()
        .split(',').map(|n| n.parse().unwrap())
        .collect();

    let other_tickets = sections.next().unwrap()
        .lines().skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Input {
        rules,
        ticket,
        other_tickets,
    }
}

fn is_field_possible(ranges: &[RangeInclusive<i64>], n: i64) -> bool {
    ranges.iter().any(|r| r.contains(&n))
}

fn is_value_valid(rules: &TicketRules, n: i64) -> bool {
    // is any rule valid?
    rules.iter().any(|(_, ranges)| {
        // is any range of this field valid?
        is_field_possible(ranges, n)
    })
}

fn part1(input: &Input) {
    let sum: i64 = input.other_tickets.iter().flat_map(|ticket| {
        // find invalid values
        ticket.iter().filter(|&&n| !is_value_valid(&input.rules, n))
    }).sum();

    println!("{}", sum);
}

fn is_ticket_valid(rules: &TicketRules, ticket: &[i64]) -> bool {
    ticket.iter().all(|&n| is_value_valid(rules, n))
}

fn part2(input: &Input) {
    let valid_tickets: Vec<_> = input.other_tickets.iter()
        .filter(|&ticket| is_ticket_valid(&input.rules, ticket)).collect();

    let num_fields = input.rules.keys().count();

    // maps fields to histograms of
    // `ticket value index` <=> `# of tickets where this field at this index is valid`
    let mut histograms: HashMap<&String, Vec<usize>> = HashMap::new();

    for (field, ranges) in input.rules.iter() {
        let histogram = histograms.entry(field)
            .or_insert_with(|| vec![0; num_fields]);

        for (index, count) in histogram.iter_mut().enumerate() {
            // Count tickets where the value is valid for this field in the current position
            *count += valid_tickets.iter()
                .filter(|&&ticket| is_field_possible(ranges, ticket[index]))
                .count();
        }
    }

    let max_count = valid_tickets.len();
    let mut index_map = HashMap::new();

    while index_map.len() != num_fields {
        for (field, hist) in histograms.iter() {
            let mut best_index = usize::MAX;
            let possible_indices = hist.iter().enumerate().filter(|(index, &count)| {
                if count == max_count && !index_map.contains_key(index) {
                    best_index = *index;
                    return true
                }
                false
            }).count();

            if possible_indices == 1 {
                index_map.insert(best_index, field);
            }
        }
    }

    let total: i64 = index_map.iter().filter_map(|(index, field)| {
        if field.starts_with("departure") {
            Some(input.ticket[*index])
        } else {
            None
        }
    }).product();

    println!("{}", total);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let input = parse(&input);

    part1(&input);
    part2(&input);
}
