use lib::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Antenna {
    pos: Pos,
    id: char,
}

fn main() {
    let input: String = lib::read_input!();
    let grid = to_grid(&input);

    let mut antennae = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                antennae.push(Antenna {
                    pos: (x as isize, y as isize).into(),
                    id: c,
                });
            }
        }
    }

    p1!(solve(&grid, &antennae, calculate_antinodes));
    p2!(solve(&grid, &antennae, calculate_antinodes_multi));
}

fn solve(
    grid: &[Vec<char>],
    antennae: &[Antenna],
    calculator: impl Fn(&[Vec<char>], Pos, Pos) -> Vec<Pos>,
) -> usize {
    let mut calculated = HashSet::new();
    let mut antinodes = HashSet::new();

    for antenna in antennae {
        if !calculated.insert(antenna) {
            continue;
        }

        for op in antennae {
            if op.pos == antenna.pos || op.id != antenna.id {
                continue;
            }

            for a in calculator(grid, antenna.pos, op.pos) {
                antinodes.insert(a);
            }
        }
    }

    antinodes.len()
}

fn calculate_antinodes(grid: &[Vec<char>], a: Pos, b: Pos) -> Vec<Pos> {
    let mut res = Vec::with_capacity(2);

    let v = b - a;

    let ap: Pos = a - v;
    if is_in_bounds(grid, ap) {
        res.push(ap);
    }

    let bp = b + v;
    if is_in_bounds(grid, bp) {
        res.push(bp);
    }

    res
}

fn calculate_antinodes_multi(grid: &[Vec<char>], a: Pos, b: Pos) -> Vec<Pos> {
    let mut res = Vec::with_capacity(2);

    let v = b - a;

    let mut lastp = a - v;
    while is_in_bounds(grid, lastp) {
        res.push(lastp);
        lastp -= v;
    }

    let mut lastp = a;
    while is_in_bounds(grid, lastp) {
        res.push(lastp);
        lastp += v;
    }

    res
}
