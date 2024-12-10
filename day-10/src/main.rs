use lib::*;
use std::{collections::HashSet, hash::Hash};

trait Counter<V> {
    fn add(&mut self, v: V);
    fn count(&self) -> usize;
}

#[derive(Default)]
struct GenericCounter {
    count: usize,
}

impl<V> Counter<V> for GenericCounter {
    fn add(&mut self, _: V) {
        self.count += 1;
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[derive(Default)]
struct UniqueCounter<V> {
    set: HashSet<V>,
}

impl<V> Counter<V> for UniqueCounter<V>
where
    V: Hash + PartialEq + Eq,
{
    fn add(&mut self, v: V) {
        self.set.insert(v);
    }

    fn count(&self) -> usize {
        self.set.len()
    }
}

fn main() {
    let input = lib::read_input!();

    let grid: Vec<Vec<usize>> = to_grid(&input)
        .into_iter()
        .map(|row| row.into_iter().map(|c| c as usize - 48).collect())
        .collect();

    p1!(trails_score::<UniqueCounter<Pos>>(&grid));
    p2!(trails_score::<GenericCounter>(&grid));
}

fn trails_score<C>(grid: &[Vec<usize>]) -> usize
where
    C: Default + Counter<Pos>,
{
    let start_positions = find_all(&grid, &0);
    start_positions
        .iter()
        .map(|p| find_trails::<C>(&grid, *p))
        .sum()
}

fn find_trails<C>(grid: &[Vec<usize>], start: Pos) -> usize
where
    C: Default + Counter<Pos>,
{
    let mut queue = vec![(start, 0)];
    let mut counter = C::default();

    loop {
        let Some((next_pos, val)) = queue.pop() else {
            break;
        };

        if val == 9 {
            counter.add(next_pos);
            continue;
        }

        for d in Direction::all() {
            let p = next_pos.mv(d);
            match get_at(&grid, p) {
                Some(v) if *v == val + 1 => queue.insert(0, (p, *v)),
                _ => continue,
            }
        }
    }

    counter.count()
}
