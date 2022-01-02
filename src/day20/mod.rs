use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::utils::ParseError;

type C = isize;
type Coords = (C, C, C);

fn parse_coords(s: &str) -> Result<Coords, ParseError> {
    let comp = s.split(",")
        .map(|v| v.trim())
        .map(|v| v.parse::<isize>())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    if comp.len() != 3 {
        Err(ParseError::new("Invalid number of coordinates"))?;
    }

    Ok((comp[0], comp[1], comp[2]))
}

fn manhattan(c: Coords) -> usize {
    (c.0.abs() + c.1.abs() + c.2.abs()) as usize
}

#[derive(Debug, Clone)]
struct Particle {
    pos: Coords,
    vel: Coords,
    acc: Coords,
}

impl FromStr for Particle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^p=<(?P<pos>.*)>, v=<(?P<vel>.*)>, a=<(?P<acc>.*)>$").unwrap();
        }

        let (pos, vel, acc) = RE.captures(s).and_then(|cap| {
            let pos = cap.name("pos").map(|v| parse_coords(v.as_str()).ok())??;
            let vel = cap.name("vel").map(|v| parse_coords(v.as_str()).ok())??;
            let acc = cap.name("acc").map(|v| parse_coords(v.as_str()).ok())??;

            Some((pos, vel, acc))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::new(pos, vel, acc))
    }
}

#[aoc_generator(day20)]
fn get_input(input: &str) -> Result<Vec<Particle>, ParseError> {
    input
        .lines()
        .map(|l| Particle::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

impl Particle {
    fn new(pos: Coords, vel: Coords, acc: Coords) -> Self {
        Self { pos, vel, acc }
    }

    fn tick(&mut self) {
        self.vel.0 += self.acc.0;
        self.vel.1 += self.acc.1;
        self.vel.2 += self.acc.2;

        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }
}

#[aoc(day20, part1)]
fn problem1(input: &Vec<Particle>) -> Result<usize, ParseError> {
    let mut histogram = HashMap::new();
    let len = input.len();
    let mut particles = input.clone();
    for _ in 0..10_000 {
        for i in 0..len {
            particles[i].tick();
            let m = manhattan(particles[i].pos);
            histogram.entry(i).and_modify(|v| *v += m).or_insert(m);
        }
    }

    let min = histogram.iter().map(|(_, v)| v).min().ok_or(ParseError::new("Could not determine min"))?;
    let part = histogram.iter().filter(|(_, v)| **v == *min).map(|(k, _)| k).next().ok_or(ParseError::new("Could not find min"))?;

    Ok(*part)
}

#[aoc(day20, part2)]
fn problem2(input: &Vec<Particle>) -> Result<usize, ParseError> {
    let len = input.len();
    let mut particles = input.clone();
    let mut destroyed = HashSet::new();
    for _ in 0..10_000 {
        let mut collision_map = HashMap::new();
        for i in 0..len {
            if destroyed.contains(&i) {
                continue;
            }

            particles[i].tick();
            collision_map.entry(particles[i].pos)
                .and_modify(|v: &mut Vec<usize>| v.push(i))
                .or_insert(vec![i]);
        }

        for (_, v) in collision_map {
            if v.len() > 1 {
                for k in v {
                    destroyed.insert(k);
                }
            }
        }
    }

    Ok(particles.len() - destroyed.len())
}
