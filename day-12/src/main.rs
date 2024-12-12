use lib::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Area {
    id: char,
    perimeter_positions: Vec<Pos>,
    area_positions: Vec<Pos>,
}

impl Area {
    fn walk(grid: &[Vec<char>], start_pos: Pos) -> Self {
        let id = get_at(grid, start_pos)
            .expect("start pos in grid bounds")
            .clone();

        let mut seen = HashSet::new();
        let mut queue = vec![start_pos];
        let mut area_positions = vec![];
        let mut perimeter_positions = vec![];

        loop {
            let Some(pos) = queue.pop() else {
                break;
            };

            let seen = !seen.insert(pos);

            match get_at(grid, pos) {
                Some(v) if !seen && v == &id => area_positions.push(pos),
                Some(v) if v != &id => {
                    perimeter_positions.push(pos);
                    continue;
                }
                None => {
                    perimeter_positions.push(pos);
                    continue;
                }
                _ => continue,
            }

            Direction::all()
                .into_iter()
                .map(|d| pos.mv(d))
                .for_each(|p| queue.insert(0, p));
        }

        Self {
            id,
            area_positions,
            perimeter_positions,
        }
    }

    fn fence_cost(&self) -> usize {
        self.area_positions.len() * self.perimeter_positions.len()
    }
}

fn main() {
    let input: String = lib::read_input!();

    let grid = to_grid(&input);

    let mut area_positions = HashSet::new();
    let mut cost = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let pos = (x as isize, y as isize).into();

            if area_positions.contains(&pos) {
                continue;
            }

            let area = Area::walk(&grid, pos);
            cost += area.fence_cost();

            area.area_positions.iter().for_each(|p| {
                area_positions.insert(*p);
            });
        }
    }

    p1!(cost);
}
