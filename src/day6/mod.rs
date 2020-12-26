use crate::utils::ParseError;
use std::num::ParseIntError;
use std::cmp::Ordering;
use crate::utils::Error;

fn parse_line(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split('\t')
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn get_input() -> Result<Vec<i32>, ParseError> {
    let input = include_str!("./input");

    input
        .lines()
        .map(|l| parse_line(l))
        .take(1)
        .next()
        .ok_or(ParseError::new("Not enough input to parse a line."))?
        .map_err(|_| ParseError::new("Could not convert string to i32."))
}

fn position_of_largest_bank(mem: &Vec<i32>) -> Option<usize> {
    mem.iter()
        .enumerate()
        .max_by(|(ia, a), (ib, b)| {
            let content_order = a.cmp(b);
            if content_order == Ordering::Equal {
                ib.cmp(ia)
            } else {
                content_order
            }
        })
        .map(|(i, _)| i)
}

fn redistribute(mut pos: usize, mem: &mut Vec<i32>) {
    let mut amount = mem[pos];
    mem[pos] = 0;
    let len = mem.len();

    while amount > 0 {
        pos = (pos + 1) % len;
        mem[pos] += 1;
        amount -= 1;
    }
}

fn repeated_redistribution(input: &Vec<i32>) -> (usize, usize) {
    let mut visited = vec![];
    let mut count = 0;
    let mut current = input.clone();
    while !visited.contains(&current) {
        visited.push(current.clone());
        if let Some(max) = position_of_largest_bank(&current) {
            redistribute(max, &mut current);
        }
        count += 1;
    }

    let cycle = if let Some(v) = visited.iter().position(|v| v == &current) {
        count - v
    } else {
        0
    };

    (count, cycle)
}

pub fn problem1() -> Result<(), ParseError> {
    let input = get_input()?;

    let (result, _) = repeated_redistribution(&input);
    println!("6/1: # of iterations: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), Error> {
    let input = get_input()?;

    let (_, cycle) = repeated_redistribution(&input);
    println!("6/2: cycle size: {}", cycle);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let input = vec![0, 2, 7, 0];
        assert_eq!(5, repeated_redistribution(&input));
    }

    #[test]
    pub fn redistribute_1() {
        let mut mem = vec![0, 2, 7, 0];
        redistribute(2, &mut mem);
        assert_eq!(vec![2, 4, 1, 2], mem);
    }

    #[test]
    pub fn position_of_largest_bank_finds_largest() {
        let mem = vec![3, 5, 2, 1];
        assert_eq!(Some(1), position_of_largest_bank(&mem));
    }

    #[test]
    pub fn position_of_largest_bank_returns_none_for_empty() {
        let mem = vec![];
        assert_eq!(None, position_of_largest_bank(&mem));
    }

    #[test]
    pub fn position_of_largest_bank_returns_first_if_tied() {
        let mem = vec![1, 4, 4, 3];
        assert_eq!(Some(1), position_of_largest_bank(&mem));
    }
}
