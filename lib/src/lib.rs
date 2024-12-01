mod direction;
mod grid;
mod pos;
mod pos3d;
mod vector;

pub use direction::*;
pub use grid::*;
pub use pos::*;
pub use pos3d::*;
pub use vector::*;

/// Reads input either from `input.txt` (or `test_input.txt` when `--test` flag is provided)
/// in the folder of the executed day project and returns the space trimmed contents as string.
#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::Read;

        let inpt_path = if std::env::args().find(|a| a == "--test").is_some() {
            "test_input.txt"
        } else {
            "input.txt"
        };

        let path = format!("{}/{}", env!("CARGO_PKG_NAME"), inpt_path);
        let mut input_file = std::fs::File::open(&path).expect("input file");
        let mut input = String::new();
        input_file.read_to_string(&mut input).expect("read input");
        input.trim_end().to_owned()
    }};
}

/// Prints the passed expression result as 'Part 1 Solution: {p}'.
#[macro_export]
macro_rules! p1 {
    ($p: expr) => {
        println!("Part 1 Solution: {}", $p);
    };
}

/// Prints the passed expression result as 'Part 2 Solution: {p}'.
#[macro_export]
macro_rules! p2 {
    ($p: expr) => {
        println!("Part 2 Solution: {}", $p);
    };
}
