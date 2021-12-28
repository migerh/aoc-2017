use crate::utils::ParseError;
use std::iter::FromIterator;

type Passphrase = Vec<String>;
type OwnedPassphrase = Vec<String>;

#[aoc_generator(day4)]
fn get_input(input: &str) -> Vec<Passphrase> {
    input
        .lines()
        .map(|p| p.split(' ').map(|s| s.to_owned()).collect::<Vec<_>>())
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

#[aoc(day4, part1)]
pub fn problem1(input: &Vec<Passphrase>) -> Result<usize, ParseError> {
    let result = input.iter()
        .map(|p| is_valid(p))
        .filter(|v| *v)
        .count();

    Ok(result)
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

#[aoc(day4, part2)]
pub fn problem2(input: &Vec<Passphrase>) -> Result<usize, ParseError> {
    let result = input.iter()
        .map(|p| is_valid_considering_anagrams(&p))
        .filter(|v| *v)
        .count();

    Ok(result)
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
        let p = vec!["abcde".to_owned(), "fghij".to_owned()];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_2() {
        let p = vec!["abcde".to_owned(), "xyz".to_owned(), "ecdab".to_owned()];
        assert_eq!(false, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_3() {
        let p = vec!["a".to_owned(), "ab".to_owned(), "abc".to_owned(), "abf".to_owned(), "abj".to_owned()];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_4() {
        let p = vec!["iiii".to_owned(), "oiii".to_owned(), "ooii".to_owned(), "oooi".to_owned()];
        assert_eq!(true, is_valid_considering_anagrams(&p));
    }

    #[test]
    pub fn example_2_5() {
        let p = vec!["oiii".to_owned(), "ioii".to_owned(), "iioi".to_owned(), "iiio".to_owned()];
        assert_eq!(false, is_valid_considering_anagrams(&p));
    }
}
