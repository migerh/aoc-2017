use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day10)]
fn get_input(input: &str) -> Result<Vec<usize>, ParseError> {
    Ok(input
        .split(",")
        .map(|v| v.parse::<usize>())
        .collect::<Result<Vec<_>, ParseIntError>>()?)
}

fn init(size: usize) -> Vec<usize> {
    (vec![0; size]).into_iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>()
}

fn tick(v: &Vec<usize>, pos: usize, length: usize) -> Vec<usize> {
    let v_len = v.len();
    let iter_rot = v.iter();
    let iter_copy = v.iter();

    let rot = iter_rot.cycle().skip(pos).take(length).collect::<Vec<_>>().into_iter().rev();
    let copy = iter_copy.cycle().skip(pos + length).take(v_len - length);

    rot.chain(copy).cycle().skip(v_len - pos).take(v_len).cloned().collect::<Vec<_>>()
}

#[aoc(day10, part1)]
fn problem1(input: &Vec<usize>) -> Result<usize, ParseError> {
    let mut vec = init(256);
    let mut pos = 0;
    let mut skip = 0;

    for l in input {
        vec = tick(&vec, pos, *l);
        pos = (pos + *l + skip) % vec.len();
        skip += 1;
    }

    Ok(vec[0] * vec[1])
}

#[aoc(day10, part2)]
fn problem2(input: &Vec<usize>) -> Result<usize, ParseError> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        assert_eq!(0, 0);
    }
}
