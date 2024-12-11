use lib::*;
use std::collections::HashMap;

fn main() {
    let input: String = lib::read_input!();

    let start_stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse"))
        .collect();

    p1!(solve(&start_stones, 25));
    p2!(solve(&start_stones, 75));
}

fn solve(stones: &[usize], n: usize) -> usize {
    let mut res_stones: usize = 0;
    let mut cache = HashMap::new();

    for &stone in stones {
        res_stones += transform_stone(stone, n, &mut cache);
    }

    res_stones
}

fn transform_stone(
    stone: usize,
    rounds_left: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if rounds_left == 0 {
        return 1;
    }

    if let Some(&r) = cache.get(&(stone, rounds_left)) {
        return r;
    }

    let mut res = 0;

    if stone == 0 {
        return transform_stone(stone + 1, rounds_left - 1, cache);
    }

    let digits = format!("{stone}");
    if digits.len() % 2 == 1 {
        return transform_stone(stone * 2024, rounds_left - 1, cache);
    }

    let (l, r) = digits.split_at(digits.len() / 2);
    res += transform_stone(l.parse().unwrap(), rounds_left - 1, cache);
    res += transform_stone(r.parse().unwrap(), rounds_left - 1, cache);

    cache.insert((stone, rounds_left), res);
    res
}
