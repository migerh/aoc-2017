use std::cmp::min;
use crate::utils::Error;

fn find_squares(target: i32) -> Option<(i32, i32)> {
    if target == 1 {
        return Some((0, 1));
    }

    let upper_bound = (1..2).cycle().enumerate()
        .skip(1)
        .map(|(i, _)| i as i32)
        .step_by(2)
        .find(|v| v*v >= target);

    if let Some(upper_bound) = upper_bound {
        let lower_bound = upper_bound - 2;
        Some((lower_bound, upper_bound))
    } else {
        None
    }
}

fn translate_coordinates(target: i32) -> Option<(i32, i32)> {
    let squares = find_squares(target);
    let (lower, upper) = if let Some((lower, upper)) = squares {
        (lower, upper)
    } else {
        return None;
    };

    let us = upper * upper;
    let ls = lower * lower;

    let quadrant_length = (us - ls) / 4;
    let mut current = ls;

    let reach = (lower + upper) / 4;
    let mut coords = (reach, -reach);

    coords.1 += min(quadrant_length, target - current);
    current += quadrant_length;
    if current >= target {
        return Some(coords);
    }

    coords.0 -= min(quadrant_length, target - current);
    current += quadrant_length;
    if current >= target {
        return Some(coords);
    }

    coords.1 -= min(quadrant_length, target - current);
    current += quadrant_length;
    if current >= target {
        return Some(coords);
    }

    coords.0 += min(quadrant_length, target - current);

    Some(coords)
}

fn manhattan(coords: (i32, i32)) -> i32 {
    coords.0.abs() + coords.1.abs()
}

fn distance(target: i32) -> Option<i32> {
    if let Some(coords) = translate_coordinates(target) {
        Some(manhattan(coords))
    } else {
        None
    }
}

pub fn problem1() -> Result<(), Error> {
    let input = 347991;

    if let Some(result) = distance(input) {
        println!("3/1: distance: {}", result);
    } else {
        println!("3/1: Could not determine distance");
    }

    Ok(())
}

pub fn problem2() -> Result<(), Error> {
    let input = 347991;

    println!("input: {}", input);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn find_squares_1_returns_0_and_1() {
        assert_eq!(Some((0, 1)), find_squares(1));
    }

    #[test]
    pub fn find_squares_2_returns_1_and_3() {
        assert_eq!(Some((1, 3)), find_squares(2));
    }

    #[test]
    pub fn find_squares_9_returns_1_and_3() {
        assert_eq!(Some((1, 3)), find_squares(9));
    }

    #[test]
    pub fn find_squares_28_returns_5_and_7() {
        assert_eq!(Some((5, 7)), find_squares(28));
    }

    #[test]
    pub fn translate_coordinates_1_returns_0_0() {
        assert_eq!(Some((0, 0)), translate_coordinates(1));
    }

    #[test]
    pub fn translate_coordinates_12_returns_2_1() {
        assert_eq!(Some((2, 1)), translate_coordinates(12));
    }

    #[test]
    pub fn translate_coordinates_23_returns_0_m2() {
        assert_eq!(Some((0, -2)), translate_coordinates(23));
    }

    #[test]
    pub fn translate_coordinates_1024_returns_0_m2() {
        assert_eq!(Some((-15, 16)), translate_coordinates(1024));
    }

    #[test]
    pub fn example_1_1() {
        assert_eq!(Some(0), distance(1));
    }

    #[test]
    pub fn example_1_2() {
        assert_eq!(Some(3), distance(12));
    }

    #[test]
    pub fn example_1_3() {
        assert_eq!(Some(2), distance(23));
    }

    #[test]
    pub fn example_1_4() {
        assert_eq!(Some(31), distance(1024));
    }
}
