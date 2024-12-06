use std::collections::HashSet;

use lib::*;

fn main() {
    let input: String = lib::read_input!();
    let grid = to_grid(&input);

    // Both in the test input as well as in the real input, the start dierction is "up".
    let start_pos = find(&grid, &'^').expect("start position");

    let mut pos = start_pos;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();

    loop {
        visited.insert(pos);

        let Some(v) = get_at(&grid, pos.mv(dir)) else {
            break;
        };

        if *v == '#' {
            dir = dir.turn_cw();
            continue;
        }

        pos = pos.mv(dir);
    }

    p1!(visited.len());

    // ---- Part 2 ----

    // Tried to find a better solution to just insert obstructions where the guard
    // will actually walk up to instead of everywhere, but I kinda ran out of time
    // so this stays here until I find a better solution.

    let mut loops = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut pos = start_pos;
            let mut dir = Direction::Up;
            let mut visited = HashSet::new();

            loop {
                if !visited.insert((pos, dir)) {
                    loops += 1;
                    break;
                }

                let next_pos = pos.mv(dir);
                let Some(v) = get_at(&grid, next_pos) else {
                    break;
                };

                if *v == '#' || x as isize == next_pos.x && y as isize == next_pos.y {
                    dir = dir.turn_cw();
                    continue;
                }

                pos = pos.mv(dir);
            }
        }
    }

    p2!(loops);
}
