use std::collections::HashMap;
use crate::utils::ParseError;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Turing {
    state: State,
    band: HashMap<usize, u8>,
    pos: usize,
}

impl Turing {
    fn new() -> Self {
        let state = State::A;
        let band = HashMap::new();
        let pos = 0;

        Self { state, band, pos }
    }

    fn write(&mut self, val: u8) {
        self.band.entry(self.pos).and_modify(|v| *v = val).or_insert(val);
    }

    fn tick(&mut self) {
        use State::*;

        let current = self.band.get(&self.pos).unwrap_or(&0);
        match (&self.state, current) {
            (A, 0) => {
                self.write(1);
                self.pos += 1;
                self.state = B;
            },
            (A, 1) => {
                self.write(0);
                self.pos -= 1;
                self.state = B;
            },
            (B, 0) => {
                self.write(1);
                self.pos -= 1;
                self.state = C;
            },
            (B, 1) => {
                self.write(0);
                self.pos += 1;
                self.state = E;
            },
            (C, 0) => {
                self.write(1);
                self.pos += 1;
                self.state = E;
            },
            (C, 1) => {
                self.write(0);
                self.pos -= 1;
                self.state = D;
            },
            (D, 0) => {
                self.write(1);
                self.pos -= 1;
                self.state = A;
            },
            (D, 1) => {
                self.write(1);
                self.pos -= 1;
                self.state = A;
            },
            (E, 0) => {
                self.write(0);
                self.pos += 1;
                self.state = A;
            },
            (E, 1) => {
                self.write(0);
                self.pos += 1;
                self.state = F;
            },
            (F, 0) => {
                self.write(1);
                self.pos += 1;
                self.state = E;
            },
            (F, 1) => {
                self.write(1);
                self.pos += 1;
                self.state = A;
            },
            _ => panic!("Cannot happen"),
        }
    }
}

#[aoc(day25, part1)]
fn problem1(_: &str) -> Result<usize, ParseError> {
    let mut turing = Turing::new();

    for _ in 0..12683008 {
        turing.tick();
    }

    let checksum = turing.band.iter().map(|(_, v)| *v as usize).sum();
    Ok(checksum)
}

#[aoc(day25, part2)]
fn problem2(_: &str) -> Result<usize, ParseError> {
    Ok(0)
}
