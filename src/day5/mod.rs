use std::num::ParseIntError;
use crate::utils::ParseError;
use crate::utils::Error;

fn get_input() -> Result<Vec<i32>, ParseError> {
    let input = include_str!("./input");

    let jumps = input
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    Ok(jumps)
}

fn inc(v: i32) -> i32 {
    v + 1
}

fn jump<F>(pos: usize, jumps: &mut Vec<i32>, update: F) -> Option<usize>
    where F: Fn(i32) -> i32 {

    let next = if let Some(p) = jumps.get_mut(pos) {
        p
    } else {
        return None;
    };

    let next_pos = (pos as i32) + *next;

    *next = update(*next);

    let result = next_pos as usize;

    if result < jumps.len() {
        Some(result)
    } else {
        None
    }
}

pub fn problem1() -> Result<(), Error> {
    let mut input = get_input()?;
    let mut counter = 0;
    let mut pos = 0;

    loop {
        counter += 1;
        let next = jump(pos, &mut input, inc);

        pos = if let Some(next) = next {
            next
        } else {
            break;
        };
    }

    println!("5/1: # of jumps: {}", counter);

    Ok(())
}

fn update_part_2(v: i32) -> i32 {
    if v >= 3 {
        v - 1
    } else {
        v + 1
    }
}

pub fn problem2() -> Result<(), Error> {
    let mut input = get_input()?;
    let mut counter = 0;
    let mut pos = 0;

    loop {
        counter += 1;
        let next = jump(pos, &mut input, update_part_2);

        pos = if let Some(next) = next {
            next
        } else {
            break;
        };
    }

    println!("5/2: # of jumps: {}", counter);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let mut j = vec![0, 3, 0, 1, -3];

        let next = jump(0, &mut j, inc);

        assert_eq!(Some(0), next);
        assert_eq!(vec![1, 3, 0, 1, -3], j);
    }

    #[test]
    pub fn example_1_2() {
        let mut j = vec![1, 3, 0, 1, -3];

        let next = jump(0, &mut j, inc);

        assert_eq!(Some(1), next);
        assert_eq!(vec![2, 3, 0, 1, -3], j);
    }

    #[test]
    pub fn example_1_3() {
        let mut j = vec![2, 4, 0, 1, -2];

        let next = jump(1, &mut j, inc);

        assert_eq!(None, next);
        assert_eq!(vec![2, 5, 0, 1, -2], j);
    }

    #[test]
    pub fn example_2_1() {
        let mut j = vec![2, 3, 0, 1, -3];

        let next = jump(1, &mut j, update_part_2);

        assert_eq!(Some(4), next);
        assert_eq!(vec![2, 2, 0, 1, -3], j);
    }
}
