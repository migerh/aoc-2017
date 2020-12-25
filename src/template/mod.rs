use crate::utils::Error;

fn get_input() -> Result<&'static str, Error> {
    let input = include_str!("./input");

    Ok(input)
}

pub fn problem1() -> Result<(), Error> {
    let input = get_input()?;

    println!("input: {}", input);

    Ok(())
}

pub fn problem2() -> Result<(), Error> {
    let input = get_input()?;

    println!("input: {}", input);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        assert_eq!(0, 0);
    }
}
