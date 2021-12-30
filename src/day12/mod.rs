use std::collections::HashSet;
use std::num::ParseIntError;
use std::collections::HashMap;
use crate::utils::ParseError;

#[aoc_generator(day12)]
fn get_input(input: &str) -> Result<HashMap<usize, Vec<usize>>, ParseError> {
    let mut map = HashMap::new();

    for l in input.lines() {
        let split = l.split(" <-> ").collect::<Vec<_>>();
        let left = split[0].trim().parse::<usize>()?;
        let mut right = split[1].trim().split(",").map(|v| v.trim().parse::<usize>()).collect::<Result<Vec<_>, ParseIntError>>()?;

        map.entry(left).and_modify(|v: &mut Vec<usize>| v.append(&mut right)).or_insert(right);
    }

    Ok(map)
}

#[aoc(day12, part1)]
fn problem1(input: &HashMap<usize, Vec<usize>>) -> Result<usize, ParseError> {
    let mut group = HashSet::new();
    let mut queue = vec![0];

    while let Some(q) = queue.pop() {
        if group.contains(&q) {
            continue;
        }

        group.insert(q);

        if let Some(next) = input.get(&q) {
            let mut next = next.clone();
            queue.append(&mut next);
        }
    }

    Ok(group.len())
}

#[aoc(day12, part2)]
fn problem2(input: &HashMap<usize, Vec<usize>>) -> Result<usize, ParseError> {
    let mut groups = vec![];
    let mut queue;
    let mut programs = input.keys().cloned().collect::<HashSet<_>>();

    while programs.len() > 0 {
        queue = vec![*programs.iter().next().ok_or(ParseError::new("Expected at least one entry"))?];
        let mut group = HashSet::new();

        while let Some(q) = queue.pop() {
            if group.contains(&q) {
                continue;
            }

            group.insert(q);

            if let Some(next) = input.get(&q) {
                let mut next = next.clone();
                queue.append(&mut next);
            }
        }

        programs = programs.difference(&group).cloned().collect::<HashSet<_>>();
        groups.push(group);
    }

    Ok(groups.len())
}
