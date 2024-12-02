#![feature(iter_map_windows)]

use lib::*;

fn main() {
    let input: String = lib::read_input!();

    let records: Vec<Vec<isize>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();

    let safe_records = records.iter().filter(|v| is_safe_record(v, false)).count();
    p1!(safe_records);

    let safe_records = records.iter().filter(|v| is_safe_record(v, true)).count();
    p2!(safe_records);
}

fn is_safe_record(record: &[isize], tolerance: bool) -> bool {
    assert!(record.len() > 2);

    let safe = is_safe(record, None);
    if safe {
        return true;
    }

    if !tolerance {
        return false;
    }

    for i in 0..record.len() {
        if is_safe(record, Some(i)) {
            return true;
        }
    }

    false
}

fn is_safe(record: &[isize], skip: Option<usize>) -> bool {
    assert!(record.len() > 2);

    let iter: Box<dyn Iterator<Item = &isize>> = match skip {
        None => Box::new(record.iter()),
        Some(i) => Box::new(record[..i].iter().chain(record[i + 1..].iter())),
    };

    let diffs: Vec<_> = iter.map_windows(|[l, r]| *l - *r).collect();

    diffs.iter().all(|d| (1..=3).contains(d)) || diffs.iter().all(|d| (-3..=-1).contains(d))
}

#[cfg(test)]
mod test {

    #[test]
    fn is_failed() {
        assert!(super::is_safe(&[1, 2, 3, 4, 5, 6], None));

        assert!(!super::is_safe(&[1, 3, 2, 4, 5], None));
        assert!(super::is_safe(&[1, 3, 2, 4, 5], Some(1)));

        assert!(!super::is_safe(&[8, 6, 4, 4, 1], None));
        assert!(super::is_safe(&[1, 3, 2, 4, 5], Some(2)));

        assert!(!super::is_safe(&[1, 7, 3, 4, 5], None));
        assert!(super::is_safe(&[1, 7, 3, 4, 5], Some(1)));
    }

    #[test]
    fn is_failed_record() {
        assert!(super::is_safe_record(&[1, 2, 3, 4, 5, 6], true));
        assert!(super::is_safe_record(&[1, 3, 2, 4, 5], true));
        assert!(super::is_safe_record(&[7, 6, 4, 2, 1], true));

        assert!(!super::is_safe_record(&[1, 2, 7, 8, 9], true));
        assert!(!super::is_safe_record(&[9, 7, 6, 2, 1], true));

        assert!(super::is_safe_record(&[1, 7, 3, 4, 5], true));
    }
}
