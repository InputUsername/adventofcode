use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operand {
    Value(u16),
    Name(String),
}

impl Operand {
    fn eval(&self, connections: &[Connection], wires: &mut HashMap<String, u16>) -> u16 {
        match self {
            Self::Value(n) => *n,
            Self::Name(x) => signal(connections, find(connections, x), wires),
        }
    }
}

impl From<String> for Operand {
    fn from(s: String) -> Self {
        s.parse().map_or_else(|_| Operand::Name(s), Operand::Value)
    }
}

#[derive(Debug, Clone)]
enum Op {
    Copy(Operand),
    Not(String),
    And(Operand, Operand),
    Or(Operand, Operand),
    LShift(Operand, Operand),
    RShift(Operand, Operand),
}

#[derive(Debug, Clone)]
struct Connection {
    op: Op,
    target: String,
}

impl std::str::FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let mut op = parts.next().unwrap().split(' ').map(|s| s.to_owned());
        let op = match (op.next(), op.next(), op.next()) {
            (Some(x), None, None) => Op::Copy(x.into()),
            (Some(op), Some(x), None) if op == "NOT" => Op::Not(x),
            (Some(x), Some(op), Some(y)) => {
                let x = x.into();
                let y = y.into();
                if op == "AND" {
                    Op::And(x, y)
                } else if op == "OR" {
                    Op::Or(x, y)
                } else if op == "LSHIFT" {
                    Op::LShift(x, y)
                } else if op == "RSHIFT" {
                    Op::RShift(x, y)
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        };

        let target = parts.next().unwrap().to_owned();

        Ok(Self { op, target })
    }
}

fn find<'c>(connections: &'c [Connection], target: &str) -> &'c Connection {
    connections.iter().find(|c| c.target == target).unwrap()
}

fn signal(connections: &[Connection], conn: &Connection, wires: &mut HashMap<String, u16>) -> u16 {
    if let Some(&signal) = wires.get(&conn.target) {
        return signal;
    }

    let signal = match conn.op {
        Op::Copy(Operand::Value(n)) => n,
        Op::Copy(Operand::Name(ref x)) => signal(connections, find(connections, x), wires),
        Op::Not(ref x) => !signal(connections, find(connections, x), wires),
        Op::And(ref x, ref y) => x.eval(connections, wires) & y.eval(connections, wires),
        Op::Or(ref x, ref y) => x.eval(connections, wires) | y.eval(connections, wires),
        Op::LShift(ref x, ref y) => x.eval(connections, wires) << y.eval(connections, wires),
        Op::RShift(ref x, ref y) => x.eval(connections, wires) >> y.eval(connections, wires),
    };

    wires.insert(conn.target.clone(), signal);

    signal
}

fn find_mut<'c>(connections: &'c mut [Connection], target: &str) -> &'c mut Connection {
    connections.iter_mut().find(|c| c.target == target).unwrap()
}

fn main() {
    let input: Vec<Connection> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(input: &[Connection]) {
    println!("{}", signal(input, find(input, "a"), &mut HashMap::new()));
}

fn part2(input: &[Connection]) {
    let mut input = input.to_vec();
    let a = signal(&input, find(&input, "a"), &mut HashMap::new());
    let b = find_mut(&mut input, "b");
    b.op = Op::Copy(Operand::Value(a));

    let a = signal(&input, find(&input, "a"), &mut HashMap::new());

    println!("{}", a);
}
