use lib::*;

fn main() {
    let input: String = lib::read_input!();
    let grid = to_matrix(&input);

    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 'X' {
                continue;
            }

            for (rx, ry) in find_around(&grid, x as isize, y as isize, 'M') {
                let (dx, dy) = (rx - x as isize, ry - y as isize);

                let Some('A') = get_at(&grid, rx + dx, ry + dy) else {
                    continue;
                };

                let Some('S') = get_at(&grid, rx + 2 * dx, ry + 2 * dy) else {
                    continue;
                };

                count += 1;
            }
        }
    }

    p1!(count);

    // ---- Part 2 ----

    let mut count = 0;

    for y in 1..grid.len() - 1 {
        'outer: for x in 1..grid[0].len() - 1 {
            if grid[y][x] != 'A' {
                continue;
            }

            let rms = find_around_vert(&grid, x as isize, y as isize, 'M');
            if rms.len() != 2 {
                continue;
            }

            for (rx, ry) in rms {
                let (dx, dy) = (rx - x as isize, ry - y as isize);

                let Some('S') = get_at(&grid, x as isize - dx, y as isize - dy) else {
                    continue 'outer;
                };
            }

            count += 1;
        }
    }

    p2!(count);
}

fn get_at(grid: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    if x.is_negative() || y.is_negative() || x >= grid[0].len() as isize || y >= grid.len() as isize
    {
        None
    } else {
        Some(grid[y as usize][x as usize])
    }
}

fn find_around_dirs(
    grid: &[Vec<char>],
    dirs: &[(isize, isize)],
    x: isize,
    y: isize,
    target: char,
) -> Vec<(isize, isize)> {
    let size_x = grid[0].len() as isize;
    let size_y = grid.len() as isize;

    dirs.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(x, y)| !x.is_negative() && !y.is_negative() && x < &size_x && y < &size_y)
        .filter(|(x, y)| grid[*y as usize][*x as usize] == target)
        .collect()
}

fn find_around(grid: &[Vec<char>], x: isize, y: isize, target: char) -> Vec<(isize, isize)> {
    find_around_dirs(
        grid,
        &[
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ],
        x,
        y,
        target,
    )
}

fn find_around_vert(grid: &[Vec<char>], x: isize, y: isize, target: char) -> Vec<(isize, isize)> {
    find_around_dirs(grid, &[(1, 1), (-1, 1), (1, -1), (-1, -1)], x, y, target)
}
