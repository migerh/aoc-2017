use std::collections::VecDeque;
use crate::utils::ParseError;

#[aoc_generator(day17)]
fn get_input(input: &str) -> Result<isize, ParseError> {
    Ok(input.parse::<isize>()?)
}

#[aoc(day17, part1)]
fn problem1(input: &isize) -> Result<usize, ParseError> {
    let length = *input as usize;
    let mut v = VecDeque::new();
    v.push_front(0);

    for i in 1..=2017 {
        let len = v.len();
        v.rotate_left(length % len + 1);
        v.push_front(i);
    }

    let result = v[1];
    Ok(result)
}

#[aoc(day17, part2)]
fn problem2(input: &isize) -> Result<usize, ParseError> {
    let length = *input as usize;
    let mut v = VecDeque::new();
    v.push_front(0);

    for i in 1..=50_000_000 {
        let len = v.len();
        v.rotate_left(length % len + 1);
        v.push_front(i);
    }

    let pos0 = v.iter().position(|v| *v == 0).ok_or(ParseError::new("Could not find 0"))?;
    let result = v[pos0 + 1];
    Ok(result)
}
