use std::collections::HashMap;
use regex::Regex;
use crate::utils::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Entry {
    program: String,
    weight: i32,
    subs: Vec<String>,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(.*)\s\((\d+)\)(\s->\s)?(.*)$").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        let program = cap[1].to_string();
        let weight = cap[2].parse::<i32>()?;
        let subs = cap[4].split(", ").map(|v| v.to_string()).filter(|v| !v.is_empty()).collect::<Vec<_>>();

        Ok(Self { program, weight, subs })
    }
}

#[aoc_generator(day7)]
fn parse_input(s: &str) -> Result<Vec<Entry>, ParseError> {
    s.lines()
        .map(|l| Entry::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn build_top_to_bottom_tree(entries: &Vec<Entry>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for e in entries {
        map.entry(&e.program)
            .or_insert(vec![]);

        for s in &e.subs {
            map.entry(&s)
                .and_modify(|v| v.push(&e.program))
                .or_insert(vec![&e.program]);
        }
    }

    map
}

#[aoc(day7, part1)]
fn problem1(entries: &Vec<Entry>) -> Result<String, ParseError> {
    let tree = build_top_to_bottom_tree(&entries);

    let root = tree.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| k)
        .next()
        .ok_or(ParseError::new("Could not find anode with no parents."))?;

    println!("7/1: Root node is: {}", root);

    Ok(root.to_string())
}

fn build_lookup_table(entries: &Vec<Entry>) -> HashMap<&str, (Entry, i32, i32)> {
    let mut map: HashMap<&str, (Entry, i32, i32)> = HashMap::new();

    for e in entries {
        map.entry(&e.program)
            .or_insert((e.clone(), 0, 0));
    }

    map
}

// fn score_tree(root: &tree: &mut HashMap<&str, (Entry, i32, i32)>) {

// }

#[aoc(day7, part2)]
fn problem2(entries: &Vec<Entry>) -> Result<usize, ParseError> {
    let lut = build_lookup_table(&entries);

    // println!("{:?}", lut);

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        assert_eq!(0, 0);
    }
}
