use std::num::ParseIntError;
use crate::utils::ParseError;

fn parse_part1(input: &str) -> Result<Vec<usize>, ParseError> {
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
fn problem1(input: &str) -> Result<usize, ParseError> {
    let input = parse_part1(input)?;
    let mut vec = init(256);
    let mut pos = 0;
    let mut skip = 0;

    for l in input {
        vec = tick(&vec, pos, l);
        pos = (pos + l + skip) % vec.len();
        skip += 1;
    }

    Ok(vec[0] * vec[1])
}

fn hashify(raw: &Vec<usize>) -> String {
    let mut dense = vec![];
    for i in 0..16 {
        let digit = raw.iter().skip(i * 16).take(16).fold(0, |acc, v| acc ^ v);
        dense.push(digit);
    }

    dense.iter().map(|v| format!("{:02x}", v)).collect::<String>()
}

#[aoc(day10, part2)]
fn problem2(input: &str) -> Result<String, ParseError> {
    let mut lengths = input.trim().chars().map(|c| c as usize).collect::<Vec<_>>();
    let mut append = vec![17, 31, 73, 47, 23];
    lengths.append(&mut append);

    let mut vec = init(256);
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for l in &lengths {
            vec = tick(&vec, pos, *l);
            pos = (pos + *l + skip) % vec.len();
            skip += 1;
        }
    }

    Ok(hashify(&vec))
}
