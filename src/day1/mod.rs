use crate::utils::ParseError;

#[aoc_generator(day1)]
fn load_numbers(input: &str) -> Result<Vec<u32>, ParseError> {
    input.chars()
        .filter(|c| c != &'\n' && c != &'\r')
        .map(|c| c.to_digit(10).ok_or(ParseError::new(&format!("Could not parse '{}' into a number", c))))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn sum_of_repeating_numbers(v: &Vec<u32>, lookahead: usize) -> u32 {
    let zip = v.iter().zip(v.iter().cycle().skip(lookahead));

    zip.filter_map(|(a, b)| if a == b {
            Some(a)
        } else {
            None
        })
        .sum()
}

fn sum_of_immediately_repeating_numbers(v: &Vec<u32>) -> u32 {
    sum_of_repeating_numbers(v, 1)
}

#[aoc(day1, part1)]
fn problem1(numbers: &Vec<u32>) -> Result<u32, ParseError> {
    let result = sum_of_immediately_repeating_numbers(numbers);

    Ok(result)
}

#[aoc(day1, part2)]
fn problem2(numbers: &Vec<u32>) -> Result<u32, ParseError> {
    if numbers.len() % 2 != 0 {
        panic!("Invalid input: Number of digits has to be even");
    }

    let lookahead = numbers.len() / 2;
    let result = sum_of_repeating_numbers(&numbers, lookahead);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        assert_eq!(9, sum_of_immediately_repeating_numbers(&vec![9, 1, 2, 1, 2, 9]));
    }

    #[test]
    pub fn example_1_2() {
        assert_eq!(3, sum_of_immediately_repeating_numbers(&vec![1, 1, 2, 2]));
    }

    #[test]
    pub fn example_1_3() {
        assert_eq!(4, sum_of_immediately_repeating_numbers(&vec![1, 1, 1, 1]));
    }

    #[test]
    pub fn example_1_4() {
        assert_eq!(0, sum_of_immediately_repeating_numbers(&vec![1, 2, 3, 4]));
    }

    #[test]
    pub fn example_2_1() {
        assert_eq!(6, sum_of_repeating_numbers(&vec![1, 2, 1, 2], 2));
    }

    #[test]
    pub fn example_2_2() {
        assert_eq!(0, sum_of_repeating_numbers(&vec![1, 2, 2, 1], 2));
    }

    #[test]
    pub fn example_2_3() {
        assert_eq!(4, sum_of_repeating_numbers(&vec![1, 2, 3, 4, 2, 5], 3));
    }

    #[test]
    pub fn example_2_4() {
        assert_eq!(12, sum_of_repeating_numbers(&vec![1, 2, 3, 1, 2, 3], 3));
    }
}
