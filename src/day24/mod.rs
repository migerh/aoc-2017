use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug, Clone, Eq, Hash)]
struct Component {
    a: isize,
    b: isize,
}

impl PartialEq for Component {
    fn eq(&self, other: &Component) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl FromStr for Component {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let ab = s.split("/").map(|c| c.parse::<isize>()).collect::<Result<Vec<_>, ParseIntError>>()?;

        if ab.len() != 2 {
            Err(ParseError::new("Need exactly two ports"))?;
        }

        Ok(Component::new(ab[0], ab[1]))
    }
}

#[aoc_generator(day24)]
fn get_input(input: &str) -> Result<Vec<Component>, ParseError> {
    input
        .lines()
        .map(|l| Component::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

impl Component {
    fn new(a: isize, b: isize) -> Self {
        Component { a, b }
    }

    fn weight(&self) -> isize {
        self.a + self.b
    }
}

fn follow_ups<'a>(next: isize, others: &'a Vec<Component>, visited: &HashSet<Component>) -> Vec<((&'a Component, isize, HashSet<Component>), isize)> {
    let mut nexts = vec![];

    for o in others {
        if !visited.contains(o) {
            if o.a == next || o.b == next {
                let nn = if o.a == next {
                    o.b
                } else {
                    o.a
                };

                let mut v = visited.clone();
                v.insert(o.clone());
                nexts.push(((o, nn, v), o.weight()));
            }
        }
    }

    nexts
}

fn trace_paths(path: &Vec<Component>, all: &Vec<Component>, open_port: isize, visited: &HashSet<Component>) -> Vec<Vec<Component>> {
    let successors = follow_ups(open_port, all, visited);

    if successors.len() == 0 {
        return vec![path.clone()];
    }

    let mut result = vec![];
    for s in successors {
        let mut p = path.clone();
        p.push(s.0.0.clone());
        let o = s.0.1;
        let v = s.0.2;
        let mut r = trace_paths(&p, all, o, &v);

        result.append(&mut r);
    }

    result
}

fn score_path(p: &Vec<Component>) -> isize {
    p.iter().map(|c| c.weight()).sum()
}

#[aoc(day24, part1)]
fn problem1(input: &Vec<Component>) -> Result<isize, ParseError> {
    let visited = HashSet::new();
    let next_val = 0;
    let p = vec![];

    let paths = trace_paths(&p, input, next_val, &visited);

    let max = paths.into_iter()
        .map(|p| score_path(&p))
        .max()
        .ok_or(ParseError::new("Could not find max"))?;

    Ok(max)
}

#[aoc(day24, part2)]
fn problem2(input: &Vec<Component>) -> Result<isize, ParseError> {
    let visited = HashSet::new();
    let next_val = 0;
    let p = vec![];

    let mut paths = trace_paths(&p, input, next_val, &visited);

    paths.sort_by(|pa, pb| pb.len().cmp(&pa.len()));
    let longest = paths[0].len();

    let max = paths.iter()
        .filter(|p| p.len() == longest)
        .map(|p| score_path(p))
        .max()
        .ok_or(ParseError::new("Could not find max"))?;

    Ok(max)
}
