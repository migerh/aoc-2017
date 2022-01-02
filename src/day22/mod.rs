use std::str::FromStr;
use std::collections::HashMap;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Status {
    Infected,
    Clean,
    Weakened,
    Flagged,
}

impl Status {
    fn new(c: char) -> Self {
        if c == '#' {
            Status::Infected
        } else {
            Status::Clean
        }
    }
}

#[derive(Debug, Clone)]
struct Carrier {
    pos: Coords,
    dir: Coords,
    nodes: HashMap<Coords, Status>,
    infected: usize,
}

impl FromStr for Carrier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let total = s.chars().count();
        let height = s.lines().count();
        let width = total / height;

        let off_y = (height / 2) as isize;
        let off_x = (width / 2) as isize;

        let nodes = s
            .lines()
            .enumerate()
            .map(move |(y, r)| r.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize - off_x, y as isize - off_y), Status::new(c))))
            .flatten()
            .collect::<HashMap<_, _>>();

        Ok(Self::new(nodes))
    }
}

impl Carrier {
    fn new(nodes: HashMap<Coords, Status>) -> Self {
        let pos = (0, 0);
        let dir = (0, -1);
        let infected = 0;

        Self { pos, dir, nodes, infected }
    }

    fn rotate_right(&mut self) {
        self.dir = match self.dir {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => (0, -1)
        };
    }

    fn rotate_left(&mut self) {
        self.dir = match self.dir {
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            _ => (0, -1)
        };
    }

    fn forward(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
    }

    fn turn_around(&mut self) {
        self.dir.0 *= -1;
        self.dir.1 *= -1;
    }

    fn burst(&mut self) {
        use Status::*;
        let status = *self.nodes.get(&self.pos).unwrap_or(&Clean);

        if status == Clean {
            self.rotate_left();
            self.nodes.entry(self.pos).and_modify(|s| *s = Infected).or_insert(Infected);
            self.forward();
            self.infected += 1;
        } else {
            self.rotate_right();
            self.nodes.entry(self.pos).and_modify(|s| *s = Clean).or_insert(Clean);
            self.forward();
        }
    }

    fn burst_v2(&mut self) {
        use Status::*;
        let status = *self.nodes.get(&self.pos).unwrap_or(&Clean);

        match status {
            Clean => {
                self.rotate_left();
                self.nodes.entry(self.pos).and_modify(|s| *s = Weakened).or_insert(Weakened);
                self.forward();
            },
            Weakened => {
                self.nodes.entry(self.pos).and_modify(|s| *s = Infected).or_insert(Infected);
                self.infected += 1;
                self.forward();
            },
            Infected => {
                self.rotate_right();
                self.nodes.entry(self.pos).and_modify(|s| *s = Flagged).or_insert(Flagged);
                self.forward();
            },
            Flagged => {
                self.turn_around();
                self.nodes.entry(self.pos).and_modify(|s| *s = Clean).or_insert(Clean);
                self.forward();
            }
        }
    }
}

#[aoc_generator(day22)]
fn get_input(input: &str) -> Result<Carrier, ParseError> {
    Carrier::from_str(input)
}

#[aoc(day22, part1)]
fn problem1(input: &Carrier) -> Result<usize, ParseError> {
    let mut carrier = input.clone();

    for _ in 0..10_000 {
        carrier.burst();
    }

    Ok(carrier.infected)
}

#[aoc(day22, part2)]
fn problem2(input: &Carrier) -> Result<usize, ParseError> {
    let mut carrier = input.clone();

    for _ in 0..10_000_000 {
        carrier.burst_v2();
    }

    Ok(carrier.infected)
}
