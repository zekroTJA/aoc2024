use std::collections::HashSet;

use lib::*;

fn main() {
    let input: String = lib::read_input!();
    let grid = to_grid(&input);

    // Both in the test input as well as in the real input, the start dierction is "down".
    let start_pos = find(&grid, &'^').expect("start position");

    let mut pos = start_pos;
    let mut dir = Direction::Down;
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);

        let new_pos = pos.mv(dir);
        let Some(v) = get_at(&grid, new_pos) else {
            break;
        };

        if *v == '#' {
            dir = dir.turn_ccw();
        }

        pos = pos.mv(dir);
    }

    p1!(visited.len());

    // ---- Part 2 ----

    let mut pos = start_pos;
    let mut dir = Direction::Down;
    let mut visited: Vec<(Pos, Direction)> = vec![];
    let mut poi = vec![];

    loop {
        let new_pos = pos.mv(dir);
        let Some(v) = get_at(&grid, new_pos) else {
            break;
        };

        if *v == '#' {
            dir = dir.turn_ccw();
        }

        pos = pos.mv(dir);

        visited.push((pos, dir));

        let ok = match dir {
            Direction::Up => visited
                .iter()
                .any(|(p, d)| d == &dir.turn_ccw() && p.y == pos.y && (..=pos.x).contains(&p.x)),
            Direction::Down => visited
                .iter()
                .any(|(p, d)| d == &dir.turn_ccw() && p.y == pos.y && (pos.x..).contains(&p.x)),
            Direction::Left => visited
                .iter()
                .any(|(p, d)| d == &dir.turn_ccw() && p.x == pos.x && (..=pos.y).contains(&p.y)),
            Direction::Right => visited
                .iter()
                .any(|(p, d)| d == &dir.turn_ccw() && p.x == pos.x && (pos.y..).contains(&p.y)),
        };
        if ok {
            poi.push(pos.mv(dir));
        }
    }

    // (3, 6) x
    // (6, 7) x
    // (7, 7) x
    // (1, 8) x
    // (3, 8) x
    // (7, 9)

    dbg!(poi.len());
}
