use crate::utils::Error;

type Passphrase<'a> = Vec<&'a str>;

fn get_input() -> Vec<Passphrase<'static>> {
    let input = include_str!("./input");

    input
        .lines()
        .map(|p| p.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn is_valid(p: &Passphrase) -> bool {
    let mut passphrase = p.clone();
    passphrase.sort();

    passphrase.iter()
        .zip(passphrase.iter().skip(1))
        .fold(true, |acc, value| acc && (value.0 != value.1))
}

pub fn problem1() -> Result<(), Error> {
    let input = get_input();

    let result = input.iter()
        .map(|p| is_valid(p))
        .filter(|v| *v)
        .count();

    println!("4/1: # of valid passphrases: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), Error> {
    let input = get_input();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let p = vec!["aa", "bb", "cc", "dd", "ee"];
        assert_eq!(true, is_valid(&p));
    }

    #[test]
    pub fn example_1_2() {
        let p = vec!["aa", "bb", "cc", "dd", "aa"];
        assert_eq!(false, is_valid(&p));
    }

    #[test]
    pub fn example_1_3() {
        let p = vec!["aa", "bb", "cc", "dd", "aaa"];
        assert_eq!(true, is_valid(&p));
    }
}
