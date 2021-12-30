use std::cmp::max;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        use Direction::*;

        Ok(match s {
            "n" => N,
            "ne" => NE,
            "se" => SE,
            "s" => S,
            "sw" => SW,
            "nw" => NW,
            _ => Err(ParseError::new("Could not parse direction"))?
        })
    }
}

#[aoc_generator(day11)]
fn get_input(input: &str) -> Result<Vec<Direction>, ParseError> {
    input.split(",")
        .filter(|v| *v != "")
        .map(|v|  Direction::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

type C = isize;
type Coords = (C, C, C);

fn manhattan(c: Coords) -> usize {
    (c.0.abs() + c.1.abs() + c.2.abs()) as usize
}

#[aoc(day11, part1)]
fn problem1(input: &Vec<Direction>) -> Result<usize, ParseError> {
    let mut c: Coords = (0, 0, 0);

    for d in input {
        use Direction::*;
        c = match d {
            N => (c.0, c.1 - 1, c.2 + 1),
            NE => (c.0 + 1, c.1 - 1, c.2),
            SE => (c.0 + 1, c.1, c.2 - 1),
            S => (c.0, c.1 + 1, c.2 - 1),
            SW => (c.0 - 1, c.1 + 1, c.2),
            NW => (c.0 -1, c.1, c.2 + 1),
        };
    }

    Ok(manhattan(c) / 2)
}

#[aoc(day11, part2)]
fn problem2(input: &Vec<Direction>) -> Result<usize, ParseError> {
    let mut c: Coords = (0, 0, 0);
    let mut furthest = 0;

    for d in input {
        use Direction::*;
        c = match d {
            N => (c.0, c.1 - 1, c.2 + 1),
            NE => (c.0 + 1, c.1 - 1, c.2),
            SE => (c.0 + 1, c.1, c.2 - 1),
            S => (c.0, c.1 + 1, c.2 - 1),
            SW => (c.0 - 1, c.1 + 1, c.2),
            NW => (c.0 -1, c.1, c.2 + 1),
        };
        furthest = max(furthest, manhattan(c));
    }

    Ok(furthest / 2)
}
