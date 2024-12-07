use lib::*;
use std::str::FromStr;

enum Op {
    Add,
    Mul,
    Combine,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Combine => format!("{a}{b}").parse().unwrap(),
        }
    }
}

struct Entry {
    result: usize,
    components: Vec<usize>,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (res, comps) = s
            .split_once(": ")
            .ok_or_else(|| "colon split failed".to_string())?;

        let result = res
            .parse()
            .map_err(|e| format!("failed parsing result: {e}"))?;

        let components = comps
            .split_ascii_whitespace()
            .map(|c| c.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("fauled parsing component: {e}"))?;

        Ok(Self { result, components })
    }
}

impl Entry {
    fn is_valid(&self, ops: &[Op]) -> bool {
        check_valid(self.components[0], &self.components[1..], self.result, ops)
    }
}

fn check_valid(curr: usize, comps: &[usize], exp: usize, ops: &[Op]) -> bool {
    if comps.is_empty() {
        return curr == exp;
    }

    ops.iter()
        .any(|op| check_valid(op.apply(curr, comps[0]), &comps[1..], exp, ops))
}

fn solve(entries: &[Entry], ops: &[Op]) -> usize {
    entries
        .iter()
        .filter(|e| e.is_valid(ops))
        .map(|e| e.result)
        .sum()
}

fn main() {
    let input: String = lib::read_input!();

    let entries = input
        .lines()
        .map(Entry::from_str)
        .collect::<Result<Vec<_>, _>>()
        .expect("entry parsing");

    p1!(solve(&entries, &[Op::Add, Op::Mul]));
    p2!(solve(&entries, &[Op::Add, Op::Mul, Op::Combine]));
}
