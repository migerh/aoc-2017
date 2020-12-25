use crate::utils::Error;

fn load_numbers() -> Result<Vec<u32>, Error> {
    let input = include_str!("./input");

    input.chars()
        .filter(|c| c != &'\n' && c != &'\r')
        .map(|c| c.to_digit(10).ok_or(Error::new(&format!("Could not parse '{}' into a number", c))))
        .collect::<Result<Vec<_>, Error>>()
}

fn sum_of_repeating_numbers(v: &Vec<u32>) -> u32 {
    let zip = v.iter().zip(v.iter().cycle().skip(1));

    zip
        .filter_map(|(a, b)| if a == b {
            Some(a)
        } else {
            None
        })
        .sum()
    }

pub fn problem1() -> Result<(), Error> {
    let numbers = load_numbers()?;
    let result = sum_of_repeating_numbers(&numbers);

    println!("1/1: sum of numbers that appear twice: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        assert_eq!(9, sum_of_repeating_numbers(&vec![9, 1, 2, 1, 2, 9]));
    }

    #[test]
    pub fn example_1_2() {
        assert_eq!(3, sum_of_repeating_numbers(&vec![1, 1, 2, 2]));
    }

    #[test]
    pub fn example_1_3() {
        assert_eq!(4, sum_of_repeating_numbers(&vec![1, 1, 1, 1]));
    }

    #[test]
    pub fn example_1_4() {
        assert_eq!(0, sum_of_repeating_numbers(&vec![1, 2, 3, 4]));
    }
}
