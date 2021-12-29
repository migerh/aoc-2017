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

fn find_root<'a>(tree: &HashMap<&'a str, Vec<&'a str>>) -> Result<&'a str, ParseError> {
    let root = tree.iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| k)
        .next()
        .ok_or(ParseError::new("Could not find a node with no parents."))?;

    Ok(root)
}

#[aoc(day7, part1)]
fn problem1(entries: &Vec<Entry>) -> Result<String, ParseError> {
    let tree = build_top_to_bottom_tree(&entries);
    let root = find_root(&tree)?;

    Ok(root.to_string())
}

fn bottom_up_tree(entries: &Vec<Entry>) -> HashMap<&str, (&Entry, usize)> {
    let mut tree = HashMap::new();

    for e in entries {
        tree.entry(e.program.as_str()).or_insert((e, 0));
    }
    tree
}

fn calculate_weights<'a>(mut entries: &mut HashMap<&'a str, (&'a Entry, usize)>, node: &'a str) -> Result<usize, ParseError> {
    let (entry, _) = entries.get(node).ok_or(ParseError::new("Could not find node"))?.clone();

    let mut sum = 0;
    for s in &entry.subs {
        sum += calculate_weights(&mut entries, s.as_str())?;
    }

    sum += entry.weight as usize;
    entries.entry(node).and_modify(|v| v.1 = sum);

    Ok(sum)
}

fn compare_weights<'a>(entries: &'a Vec<String>, tree: &'a HashMap<&'a str, (&'a Entry, usize)>) -> Result<Option<(&'a str, usize)>, ParseError> {
    let entry_weights = entries.iter()
        .map(|e| tree.get(e.as_str()).ok_or(ParseError::new("Cannot find node")))
        .collect::<Result<Vec<_>, ParseError>>()?;

    let mut histogram: HashMap<usize, Vec<&str>> = HashMap::new();

    for (e, w) in entry_weights.into_iter() {
        histogram.entry(*w).and_modify(|v| v.push(e.program.as_str())).or_insert(vec![e.program.as_str()]);
    }

    let single = histogram.iter().filter(|(_, v)| v.len() == 1).count();
    let rest: usize = histogram.iter().filter(|(_, v)| v.len() != 1).map(|(_, v)| v.len()).sum();

    if rest == entries.len() && single == 0 {
        return Ok(None);
    }

    if rest == entries.len() - 1 && single == 1 {
        let single = histogram.iter().filter(|(_, v)| v.len() == 1).map(|(_, v)| v).next().unwrap();

        let target = tree.get(single[0]).unwrap();
        let keys = histogram.keys().collect::<Vec<_>>();
        let sub = (*keys[0] as isize - *keys[1] as isize).abs() as usize;
        return Ok(Some((single[0], (target.0.weight as usize) - sub)));
    }

    Err(ParseError::new("Something went wrong"))
}

#[aoc(day7, part2)]
fn problem2(entries: &Vec<Entry>) -> Result<usize, ParseError> {
    let tree = build_top_to_bottom_tree(&entries);
    let root = find_root(&tree)?;

    let mut tree = bottom_up_tree(entries);
    let mut target = calculate_weights(&mut tree, root)?;

    let mut queue = vec![root];

    while let Some(q) = queue.pop() {
        let (node, _) = tree.get(q).ok_or(ParseError::new("Cant find node"))?;
        if let Some((next, next_target)) = compare_weights(&node.subs, &tree)? {
            target = next_target;
            queue.push(next);
        } else {
            break;
        }
    }

    Ok(target)
}
