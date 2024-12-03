use lib::*;

fn main() {
    let input: String = lib::read_input!();

    p1!(solve(&input, false));
    p2!(solve(&input, true));
}

struct Buffer {
    chars: Vec<char>,
    cursor: usize,
}

impl Buffer {
    fn new(s: &str) -> Buffer {
        Buffer {
            chars: s.chars().collect(),
            cursor: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.cursor == self.chars.len() {
            None
        } else {
            let c = self.chars[self.cursor];
            self.cursor += 1;
            Some(c)
        }
    }

    fn rev(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }
}

fn solve(input: &str, respect_instructions: bool) -> usize {
    let mut buf = Buffer::new(input);
    let mut sum = 0;
    let mut is_active = true;

    loop {
        match buf.next() {
            Some('m') => {
                buf.rev();
                let Ok(res) = parse_mul(&mut buf) else {
                    break;
                };
                if is_active {
                    if let Some((l, r)) = res {
                        sum += l * r;
                    }
                }
            }
            Some('d') => {
                buf.rev();
                let Ok(res) = parse_instruction(&mut buf) else {
                    break;
                };
                if let Some(instruction) = res {
                    is_active = !respect_instructions || instruction;
                }
            }
            Some(_) => continue,
            None => break,
        }
    }

    sum
}

fn parse_mul(buf: &mut Buffer) -> Result<Option<(usize, usize)>, ()> {
    if !expect_sequence(buf, "mul(")? {
        return Ok(None);
    }

    let Some(first_num) = parse_number(buf)? else {
        return Ok(None);
    };

    buf.rev();
    if buf.next().ok_or(())? != ',' {
        return Ok(None);
    }

    let Some(second_num) = parse_number(buf)? else {
        return Ok(None);
    };

    buf.rev();
    if buf.next().ok_or(())? != ')' {
        return Ok(None);
    }

    Ok(Some((first_num, second_num)))
}

fn parse_instruction(buf: &mut Buffer) -> Result<Option<bool>, ()> {
    if !expect_sequence(buf, "do")? {
        buf.rev();
        return Ok(None);
    }

    let c = buf.next().ok_or(())?;
    match c {
        '(' => {
            let c = buf.next().ok_or(())?;
            if c != ')' {
                buf.rev();
                return Ok(None);
            }
            Ok(Some(true))
        }
        'n' => {
            if !expect_sequence(buf, "'t()")? {
                buf.rev();
                return Ok(None);
            }
            Ok(Some(false))
        }
        _ => Ok(None),
    }
}

fn expect_sequence(buf: &mut Buffer, exp: &str) -> Result<bool, ()> {
    for ec in exp.chars() {
        let c = buf.next().ok_or(())?;
        if c != ec {
            return Ok(false);
        }
    }
    Ok(true)
}

fn parse_number(buf: &mut Buffer) -> Result<Option<usize>, ()> {
    let mut num_str = String::new();
    loop {
        let Some(c) = buf.next() else {
            return Err(());
        };

        if !c.is_ascii_digit() {
            return Ok(num_str.parse().ok());
        }

        num_str.push(c);
    }
}
