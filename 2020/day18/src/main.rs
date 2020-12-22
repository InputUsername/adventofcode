use std::fs;

#[derive(Debug)]
pub enum Token {
    Num(u64),
    Add,
    Mul,
    OpenPar,
    ClosePar,
}

fn tokenize(s: &str) -> Vec<Token> {
    s.chars().filter_map(|c| {
        Some(match c {
            ' ' => return None,
            '+' => Token::Add,
            '*' => Token::Mul,
            '(' => Token::OpenPar,
            ')' => Token::ClosePar,
           d => Token::Num(d.to_digit(10).unwrap() as u64)
        })
    }).collect()
}

mod ruleset_1 {
    use super::*;

    pub fn eval_expr(tokens: &[Token]) -> (u64, &[Token]) {
        let (lhs, rest) = eval_end(tokens);
        eval_expr_rhs(lhs, rest)
    }

    pub fn eval_expr_rhs(lhs: u64, tokens: &[Token]) -> (u64, &[Token]) {
        match tokens.first() {
            Some(Token::Add) => {
                let (rhs, rest) = eval_end(&tokens[1..]);
                eval_expr_rhs(lhs + rhs, rest)
            }
            Some(Token::Mul) => {
                let (rhs, rest) = eval_end(&tokens[1..]);
                eval_expr_rhs(lhs * rhs, rest)
            }
            _ => (lhs, tokens)
        }
    }

    pub fn eval_end(tokens: &[Token]) -> (u64, &[Token]) {
        match tokens[0] {
            Token::Num(n) => (n, &tokens[1..]),
            Token::OpenPar => {
                let (result, rest) = eval_expr(&tokens[1..]);
                (result, &rest[1..])
            }
            _ => unreachable!(),
        }
    }
}

fn part1(exprs: &[Vec<Token>]) {
    let sum: u64 = exprs.iter().map(|tokens| ruleset_1::eval_expr(tokens).0).sum();
    println!("{}", sum);
}

mod ruleset_2 {
    use super::*;

    pub fn eval_expr(tokens: &[Token]) -> (u64, &[Token]) {
        let (lhs, rest) = eval_factor(tokens);
        eval_expr_rhs(lhs, rest)
    }

    pub fn eval_expr_rhs(lhs: u64, tokens: &[Token]) -> (u64, &[Token]) {
        match tokens.first() {
            Some(Token::Mul) => {
                let (rhs, rest) = eval_factor(&tokens[1..]);
                eval_expr_rhs(lhs * rhs, rest)
            }
            _ => (lhs, tokens)
        }
    }

    pub fn eval_factor(tokens: &[Token]) -> (u64, &[Token]) {
        let (lhs, rest) = eval_term(tokens);
        eval_factor_rhs(lhs, rest)
    }

    pub fn eval_factor_rhs(lhs: u64, tokens: &[Token]) -> (u64, &[Token]) {
        match tokens.first() {
            Some(Token::Add) => {
                let (rhs, rest) = eval_term(&tokens[1..]);
                eval_factor_rhs(lhs + rhs, rest)
            }
            _ => (lhs, tokens)
        }
    }

    pub fn eval_term(tokens: &[Token]) -> (u64, &[Token]) {
        match tokens[0] {
            Token::Num(n) => (n, &tokens[1..]),
            Token::OpenPar => {
                let (result, rest) = eval_expr(&tokens[1..]);
                (result, &rest[1..])
            }
            _ => unreachable!()
        }
    }
}

fn part2(exprs: &[Vec<Token>]) {
    let sum: u64 = exprs.iter().map(|tokens| ruleset_2::eval_expr(tokens).0).sum();
    println!("{}", sum);
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let exprs: Vec<_> = input.lines().map(tokenize).collect();

    part1(&exprs);
    part2(&exprs);
}
