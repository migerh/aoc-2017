use std::collections::HashSet;
use std::collections::HashMap;
use crate::utils::ParseError;
use crate::knot::hash;

fn used(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 1,
        '3' => 2,
        '4' => 1,
        '5' => 2,
        '6' => 2,
        '7' => 3,
        '8' => 1,
        '9' => 2,
        'a' => 2,
        'b' => 3,
        'c' => 2,
        'd' => 3,
        'e' => 3,
        'f' => 4,
        _ => 0,
    }
}

#[aoc(day14, part1)]
fn problem1(input: &str) -> Result<usize, ParseError> {
    let result = (0..128)
        .map(|v| format!("{}-{}", input, v))
        .map(|v| hash(v.as_str()))
        .map(|h| h.chars().map(|c| used(c)).sum::<usize>())
        .sum::<usize>();

    Ok(result)
}

type C = isize;
type Coords = (C, C);

fn to_map(h: Vec<String>) -> Result<HashMap<Coords, char>, ParseError> {
    let v = h.into_iter()
        .map(|s| s.chars().map(|c| c.to_digit(16)).collect::<Option<Vec<_>>>())
        .collect::<Option<Vec<_>>>()
        .ok_or(ParseError::new("Unable to parse digit"))?
        .into_iter()
        .map(|v| v.into_iter().map(|c| format!("{:04b}", c).chars().collect::<Vec<_>>()).flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into_iter()
        .enumerate()
        .map(move |(y, r)| r.into_iter().enumerate().map(move |(x, v)| ((x as isize, y as isize), v)))
        .flatten()
        .collect::<HashMap<_, _>>();

    Ok(v)
}

#[aoc(day14, part2)]
fn problem2(input: &str) -> Result<usize, ParseError> {
    let result = (0..128)
        .map(|v| format!("{}-{}", input, v))
        .map(|v| hash(v.as_str()))
        .collect::<Vec<_>>();

    let mut map = to_map(result)?;
    let mut regions = vec![];
    let delta = vec![-1, 1];

    while let Some(next) = map.iter().filter(|(_, v)| **v == '1').map(|(k, _)| k).next() {
        let mut queue = vec![*next];
        let mut visited = HashSet::new();

        while let Some(q) = queue.pop() {
            if visited.contains(&q) {
                continue;
            }

            visited.insert(q);
            for dx in &delta {
                let x = q.0 + dx;
                let y = q.1;
                if let Some(v) = map.get(&(x, y)) {
                    if *v == '1' {
                        queue.push((x, y));
                    }
                }
            }

            for dy in &delta {
                let x = q.0;
                let y = q.1 + dy;
                if let Some(v) = map.get(&(x, y)) {
                    if *v == '1' {
                        queue.push((x, y));
                    }
                }
            }
        }

        for v in &visited {
            map.entry(*v).and_modify(|c| *c = '0');
        }
        regions.push(visited);
    }

    Ok(regions.len())
}
