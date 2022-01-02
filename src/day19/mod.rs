use std::str::FromStr;
use std::collections::HashMap;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C);

#[derive(Debug)]
struct Map {
    data: HashMap<Coords, char>,
    size: Coords,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Map, ParseError> {
        let data = s
            .lines()
            .enumerate()
            .map(move |(y, l)| l
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
            )
            .flatten()
            .collect::<HashMap<_, _>>();

        let mx = data.iter().map(|(k, _)| k.0 as isize).max().ok_or(ParseError::new("Could not determine size in x direction"))?;
        let my = data.iter().map(|(k, _)| k.1 as isize).max().ok_or(ParseError::new("Could not determine size in y direction"))?;
        let size = (mx, my);

        Ok(Map { data, size })
    }
}

#[aoc_generator(day19)]
fn get_input(input: &str) -> Result<Map, ParseError> {
    Map::from_str(input)
}

fn find_start(map: &Map) -> Result<Coords, ParseError> {
    let my = map.size.1;
    let start = map.data.iter()
        .filter(|(k, v)| (k.1 == my || k.1 == 0) && **v == '|')
        .map(|(k, _)| k)
        .next().ok_or(ParseError::new("Could not find start"))?;

    Ok(*start)
}

fn neighbors(map: &Map, c: &Coords) -> Vec<Coords> {
    let delta = [-1, 1];

    let mut n = vec![];
    for d in &delta {
        if let Some(v) = map.data.get(&(d + c.0, c.1)) {
            if *v != ' ' {
                n.push((d + c.0, c.1));
            }
        }

        if let Some(v) = map.data.get(&(c.0, d + c.1)) {
            if *v != ' ' {
                n.push((c.0, d + c.1));
            }
        }
    }

    n
}

fn is_path(map: &Map, c: &Coords) -> bool {
    if let Some(v) = map.data.get(c) {
        *v != ' '
    } else {
        false
    }
}

fn trace(map: &Map, start: Coords) -> Result<(Vec<char>, usize), ParseError> {
    let n = neighbors(map, &start);

    if n.len() != 1 {
        Err(ParseError::new("Could not determine start direction"))?;
    }

    let second = n[0];
    let mut dir = (second.0 - start.0, second.1 - start.1);

    let mut i = 0;
    let mut path = vec![];
    let mut prev = start.clone();
    let mut queue = vec![start];
    while let Some(q) = queue.pop() {
        i += 1;
        if let Some(v) = map.data.get(&q) {
            if v.is_alphabetic() {
                path.push(*v);
            }
        }

        let next = (q.0 + dir.0, q.1 + dir.1);
        if is_path(map, &next) {
            queue.push(next);
            prev = q;
            continue;
        }

        if let Some(n) = neighbors(map, &q).into_iter().filter(|c| *c != prev).next() {
            dir = (n.0 - q.0, n.1 - q.1);
            queue.push(n);
        } else {
            break;
        }
        prev = q;
    }

    Ok((path, i))
}

#[aoc(day19, part1)]
fn problem1(input: &Map) -> Result<String, ParseError> {
    let start = find_start(input)?;
    let (path, _) = trace(input, start)?;

    Ok(path.into_iter().collect::<String>())
}

#[aoc(day19, part2)]
fn problem2(input: &Map) -> Result<usize, ParseError> {
    let start = find_start(input)?;
    let (_, steps) = trace(input, start)?;

    Ok(steps)
}
