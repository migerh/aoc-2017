use std::num::ParseIntError;
use itertools::Itertools;
use crate::utils::ParseError;

fn parse_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
    line.split('\t')
        .map(|v| v.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn get_input() -> Result<Vec<Vec<i32>>, ParseIntError> {
    include_str!("./input")
        .lines()
        .map(|l| parse_line(l))
        .collect::<Result<Vec<_>, ParseIntError>>()
}

fn get_min_max(v: &Vec<i32>) -> Option<(i32, i32)> {
    let min = v.iter().min();
    let max = v.iter().max();

    if let (Some(&min), Some(&max)) = (min, max) {
        Some((min, max))
    } else {
        None
    }
}

fn checksum(matrix: &Vec<Vec<i32>>) -> i32 {
    matrix.iter()
        .filter_map(|v| get_min_max(v))
        .map(|(min, max)| max - min)
        .sum()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = get_input()?;

    let result = checksum(&input);
    println!("2/1: checksum is: {}", result);

    Ok(())
}

fn reduce_row(v: &Vec<i32>) -> Option<i32> {
    let combo = v.iter()
        .combinations(2)
        .map(|c| (c[0], c[1]))
        .find(|(a, b)| **b % **a == 0 || **a % **b == 0);

    if let Some((a, b)) = combo {
        if a > b {
            Some(a / b)
        } else {
            Some(b / a)
        }
    } else {
        None
    }
}

fn divider_checksum(v: &Vec<Vec<i32>>) -> i32 {
    v.iter()
        .filter_map(|r| reduce_row(r))
        .sum()
}

pub fn problem2() -> Result<(), ParseError> {
    let input = get_input()?;

    let result = divider_checksum(&input);
    println!("2/2: division based checksum: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let matrix = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8],
        ];
        assert_eq!(18, checksum(&matrix));
    }


    #[test]
    pub fn example_1_2() {
        let matrix = vec![
            vec![5, 9, 2, 8],
            vec![9, 4, 7, 3],
            vec![3, 8, 6, 5],
        ];
        assert_eq!(9, divider_checksum(&matrix));
    }
}
