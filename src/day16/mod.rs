use std::collections::VecDeque;
use regex::Regex;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug)]
enum Moves {
    Spin(usize),
    Exchange((usize, usize)),
    Partner((char, char))
}

impl FromStr for Moves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        use Moves::*;

        lazy_static!{
            static ref RE_SPIN: Regex = Regex::new(r"^s(?P<num>.*)?$").unwrap();
            static ref RE_EXCHANGE: Regex = Regex::new(r"^x(?P<A>.*)?/(?P<B>.*)$").unwrap();
            static ref RE_PARTNER: Regex = Regex::new(r"^p(?P<A>[a-z])?/(?P<B>[a-z])$").unwrap();
        }

        let first = s.chars().next().ok_or(ParseError::new("Empty string given"))?;

        Ok(match first {
            's' => {
                let cap = RE_SPIN.captures(s).ok_or(ParseError::new("Invalid spin"))?;
                let num = cap.name("num").map(|v| v.as_str().parse::<usize>()).ok_or(ParseError::new("Could not parse param"))??;
                Spin(num)
            },
            'x' => {
                let cap = RE_EXCHANGE.captures(s).ok_or(ParseError::new("Invalid exchange"))?;
                let a = cap.name("A").map(|a| a.as_str().parse::<usize>()).ok_or(ParseError::new("Could not parse param A"))??;
                let b = cap.name("B").map(|b| b.as_str().parse::<usize>()).ok_or(ParseError::new("Could not parse param B"))??;
                Exchange((a, b))
            },
            'p' => {
                let cap = RE_PARTNER.captures(s).ok_or(ParseError::new("Invalid partner"))?;
                let a = cap.name("A").map(|a| a.as_str().chars().next()).flatten().ok_or(ParseError::new("Could not parse param A"))?;
                let b = cap.name("B").map(|b| b.as_str().chars().next()).flatten().ok_or(ParseError::new("Could not parse param B"))?;
                Partner((a, b))
            },
            _ => Err(ParseError::new("Cannot parse string"))?
        })
    }
}

#[aoc_generator(day16)]
fn get_input(input: &str) -> Result<Vec<Moves>, ParseError> {
    input
        .split(",")
        .map(|v| Moves::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn init() -> VecDeque<char> {
    vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'].into_iter().collect::<VecDeque<_>>()
}

#[aoc(day16, part1)]
fn problem1(input: &Vec<Moves>) -> Result<String, ParseError> {
    let mut vec = init();

    for m in input {
        use Moves::*;

        match m {
            Spin(s) => vec.rotate_right(*s),
            Exchange((a, b)) => vec.swap(*a, *b),
            Partner((a, b)) => {
                let i = vec.iter().position(|v| v == a).ok_or(ParseError::new("Could not find element"))?;
                let j = vec.iter().position(|v| v == b).ok_or(ParseError::new("Could not find element"))?;
                vec.swap(i, j);
            }
        }
    }

    Ok(vec.into_iter().collect::<String>())
}

#[allow(unused_assignments)]
#[aoc(day16, part2)]
fn problem2(input: &Vec<Moves>) -> Result<String, ParseError> {
    let mut vec = init();
    let start = "abcdefghijklmnop";
    let mut cycle = 30;

    let target = 1_000_000_000 % cycle;

    // for i in 0..1_000_000_000 {
    for i in 0..target {
        for m in input {
            use Moves::*;

            match m {
                Spin(s) => vec.rotate_right(*s),
               Exchange((a, b)) => vec.swap(*a, *b),
                Partner((a, b)) => {
                    let i = vec.iter().position(|v| v == a).ok_or(ParseError::new("Could not find element"))?;
                    let j = vec.iter().position(|v| v == b).ok_or(ParseError::new("Could not find element"))?;
                    vec.swap(i, j);
                }
            }
        }

        if start == vec.iter().collect::<String>() {
            cycle = i + 1;
            break;
        }
    }

    Ok(vec.into_iter().collect::<String>())
}
