use lib::*;

#[derive(Clone, Debug)]
enum Element {
    Space { size: usize },
    File { id: usize, size: usize },
}

impl Element {
    pub fn size(&self) -> usize {
        match self {
            Element::Space { size } => *size,
            Element::File { id: _, size } => *size,
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let sequence: Vec<_> = input
        .bytes()
        .map(|c| c as usize - 48)
        .enumerate()
        .map(|(i, v)| {
            if i % 2 == 0 {
                Element::File { id: i / 2, size: v }
            } else {
                Element::Space { size: v }
            }
        })
        .collect();

    assert!(sequence.len() % 2 == 1);

    let mut seq = sequence.clone();
    let mut checksum = 0;
    let mut cursor = 0;

    for i in 0.. {
        let Some(elem) = seq.get(i).cloned() else {
            break;
        };

        let size = elem.size();

        for _ in 0..size {
            match elem {
                Element::Space { size: _ } => {
                    let last_idx = seq.len() - 1;
                    match seq[last_idx] {
                        Element::Space { size: _ } => panic!("should not be space"),
                        Element::File {
                            id: last_id,
                            size: last_size,
                        } => {
                            checksum += cursor * last_id;
                            if last_size == 1 {
                                seq.truncate(last_idx - 1);
                            } else {
                                seq[last_idx] = Element::File {
                                    id: last_id,
                                    size: last_size - 1,
                                };
                            }
                        }
                    }
                }
                Element::File { id, size: _ } => {
                    checksum += cursor * id;
                }
            }

            cursor += 1;
        }
    }

    p1!(checksum);

    // ---- Part 2 ----

    let mut seq = sequence.clone();

    for idx in (1..=seq.len() / 2).rev() {
        let (last_file_idx, last_file) = seq
            .iter()
            .enumerate()
            .rev()
            .find(|(_, elem)| matches!(elem, Element::File { id, size: _ } if *id == idx))
            .expect("last file");

        let Some((space_idx, space)) = seq
            .iter()
            .enumerate()
            .find(|(idx, elem)| matches!(elem, Element::Space { size } if *idx < last_file_idx && *size >= last_file.size()))
        else {
            continue;
        };

        let space_diff = space.size() - last_file.size();

        let last_file = last_file.clone();

        if space_diff == 0 {
            seq[last_file_idx] = Element::Space {
                size: last_file.size(),
            };
            seq[space_idx] = last_file;
            continue;
        }

        seq[space_idx] = Element::Space { size: space_diff };
        seq[last_file_idx] = Element::Space {
            size: last_file.size(),
        };
        seq.insert(space_idx, last_file);
    }

    let mut checksum = 0;
    let mut cursor = 0;

    for elem in seq {
        match elem {
            Element::Space { size } => cursor += size,
            Element::File { id, size } => {
                for _ in 0..size {
                    checksum += cursor * id;
                    cursor += 1;
                }
            }
        }
    }

    p2!(checksum);
}
