use lib::*;

fn main() {
    let input: String = lib::read_input!();

    let comb: Vec<_> = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .collect();

    let mut left: Vec<isize> = comb.iter().map(|(l, _)| l.parse().unwrap()).collect();
    let mut right: Vec<isize> = comb.iter().map(|(_, r)| r.parse().unwrap()).collect();

    left.sort();
    right.sort();

    let res: isize = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    p1!(res);

    // ---- Part 2 -----

    let res: isize = left
        .iter()
        .map(|l| right.iter().filter(|&r| r == l).count() as isize * l)
        .sum();

    p2!(res);
}
