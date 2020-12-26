use crate::utils::Error;
use std::iter::FromIterator;

type Passphrase<'a> = Vec<&'a str>;
type OwnedPassphrase = Vec<String>;

fn get_input() -> Vec<Passphrase<'static>> {
    let input = include_str!("./input");

    input
        .lines()
        .map(|p| p.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn is_valid<T>(p: &Vec<T>) -> bool
    where T: Clone + Ord {
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

fn normalize_word(word: &str) -> String {
    let mut w = word.chars().collect::<Vec<_>>();
    w.sort();

    String::from_iter(w.iter())
}

fn normalize(passes: &Passphrase) -> OwnedPassphrase {
    passes.iter()
        .map(|v| normalize_word(v))
        .collect::<Vec<_>>()
}

fn is_valid_considering_anagrams(p: &Passphrase) -> bool {
    let normalized = normalize(p);
    is_valid(&normalized)
}

pub fn problem2() -> Result<(), Error> {
    let input = get_input();

    let result = input.iter()
        .map(|p| is_valid_considering_anagrams(&p))
        .filter(|v| *v)
        .count();

    println!("4/2: # of valid passphrases considering anagrams: {}", result);

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

    #[test]
    pub fn example_2_1() {
        let p = vec!["abcde", "fghij"];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_2() {
        let p = vec!["abcde", "xyz", "ecdab"];
        assert_eq!(false, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_3() {
        let p = vec!["a", "ab", "abc", "abf", "abj"];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_4() {
        let p = vec!["iiii", "oiii", "ooii", "oooi"];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_5() {
        let p = vec!["oiii", "ioii", "iioi", "iiio"];
        assert_eq!(false, is_valid_considering_anagrams(&p));
    }
}
