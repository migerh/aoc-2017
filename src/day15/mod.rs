use std::num::ParseIntError;
use crate::utils::ParseError;

type Start = (usize, usize);

#[aoc_generator(day15)]
fn get_input(input: &str) -> Result<Start, ParseError> {
    let starting = input
        .lines()
        .map(|v| v.split(" with ").skip(1).take(1))
        .flatten()
        .map(|v| v.parse::<usize>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    if starting.len() == 2 {
        Ok((starting[0], starting[1]))
    } else {
        Err(ParseError::new("Could not parse input"))
    }
}

#[aoc(day15, part1)]
fn problem1(input: &Start) -> Result<usize, ParseError> {
    let factors = (16807, 48271);
    let divisor = 2147483647;
    let mask = 0b1111111111111111;

    let mut a = input.0;
    let mut b = input.1;
    let mut same = 0;

    for _ in 0..40_000_000 {
        a = (a * factors.0) % divisor;
        b = (b * factors.1) % divisor;

        if a & mask == b & mask {
            same += 1;
        }
    }

    Ok(same)
}

#[aoc(day15, part2)]
fn problem2(input: &Start) -> Result<usize, ParseError> {
    let factors = (16807, 48271);
    let divisor = 2147483647;
    let mask = 0b1111111111111111;

    let mut a = input.0;
    let mut b = input.1;
    let mut same = 0;

    for _ in 0..5_000_000 {
        a = (a * factors.0) % divisor;
        while a & 3 != 0 {
            a = (a * factors.0) % divisor;
        }

        b = (b * factors.1) % divisor;
        while b & 7 != 0 {
            b = (b * factors.1) % divisor;
        }

        if a & mask == b & mask {
            same += 1;
        }
    }

    Ok(same)
}
