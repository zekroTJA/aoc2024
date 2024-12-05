use lib::*;
use std::collections::HashMap;

fn main() {
    let input: String = lib::read_input!();

    let (rules, orders) = input.split_once("\n\n").expect("split");

    let rule_pairs: Vec<(usize, usize)> = rules
        .lines()
        .map(|l| l.split_once('|').expect("rule split"))
        .map(|(l, r)| {
            (
                l.parse().expect("rule l parse"),
                r.parse().expect("rule r parse"),
            )
        })
        .collect();

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for (l, r) in rule_pairs {
        let entry = rules.entry(l).or_default();
        entry.push(r);
    }

    let orders: Vec<Vec<usize>> = orders
        .lines()
        .map(|l| {
            l.split(',')
                .map(|e| e.parse().expect("order parse"))
                .collect()
        })
        .collect();

    let p1: usize = orders
        .iter()
        .filter(|order| is_ordered(order, &rules))
        .map(|order| get_pivot_value(order))
        .sum();
    p1!(p1);

    // ---- Part 2 ----

    let p2: usize = orders
        .iter()
        .filter(|order| !is_ordered(order, &rules))
        .map(|order| re_order(order, &rules))
        .map(|order| get_pivot_value(&order))
        .sum();

    p2!(p2);
}

fn is_ordered(order: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    for idx in 0..order.len() {
        let val = order[idx];
        let Some(before) = rules.get(&val) else {
            continue;
        };
        let min = before
            .iter()
            .filter_map(|b| order.iter().enumerate().find(|(_, v)| *v == b))
            .map(|(i, _)| i)
            .min();
        if let Some(min) = min {
            if idx > min {
                return false;
            }
        }
    }
    true
}

fn re_order(order: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut res = Vec::with_capacity(order.len());

    res.push(order[0]);

    for &v in order[1..].iter() {
        let Some(before) = rules.get(&v) else {
            res.push(v);
            continue;
        };

        let min = before
            .iter()
            .filter_map(|b| res.iter().enumerate().find(|(_, v)| *v == b))
            .map(|(i, _)| i)
            .min();

        match min {
            Some(min) => res.insert(min, v),
            None => res.push(v),
        }
    }

    res
}

fn get_pivot_value<T: Copy>(s: &[T]) -> T {
    s[s.len() / 2]
}
