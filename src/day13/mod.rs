use std::num::ParseIntError;
use std::collections::HashMap;
use crate::utils::ParseError;

#[aoc_generator(day13)]
fn get_input(input: &str) -> Result<HashMap<usize, usize>, ParseError> {
    Ok(input.lines()
        .map(|l| l.split(": ").map(|n| n.parse::<usize>()).collect::<Result<Vec<_>, ParseIntError>>())
        .collect::<Result<Vec<_>, ParseIntError>>()?
        .iter()
        .map(|e| (e[0], e[1]))
        .collect::<HashMap<_, _>>())
}

fn is_caught(layer: usize, depth: usize) -> bool {
    let cycle = 2 * depth - 2;

    layer % cycle == 0
}

fn penalty(layer: usize, depth: usize) -> usize {
    if is_caught(layer, depth) {
        layer * depth
    } else {
        0
    }
}

#[aoc(day13, part1)]
fn problem1(input: &HashMap<usize, usize>) -> Result<usize, ParseError> {
    let max = input.iter().map(|(k, _)| k).max().ok_or(ParseError::new("Could not determine max."))?;
    let mut severity = 0;

    for time in 0..=*max {
        if let Some(depth) = input.get(&time) {
            severity += penalty(time, *depth);
        }
    }

    Ok(severity)
}

#[aoc(day13, part2)]
fn problem2(input: &HashMap<usize, usize>) -> Result<usize, ParseError> {
    // let prod: usize = input.iter().map(|(_, v)| v).product();
    let max = input.iter().map(|(k, _)| k).max().ok_or(ParseError::new("Could not determine max."))?;
    let mut caught = false;
    let mut target = None;

    for delay in 0.. {
        for time in 0..=*max {
            if let Some(depth) = input.get(&time) {
                caught |= is_caught(time + delay, *depth);
            }
        }
        if !caught {
            target = Some(delay);
            break;
        }
        caught = false;
    }

    Ok(target.ok_or(ParseError::new("Could not determine delay"))?)
}
