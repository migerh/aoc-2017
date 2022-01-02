use std::fmt::{Write, Error, Display, Formatter};
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug, Clone)]
struct Pattern {
    data: Vec<Vec<char>>,
}

// Construction helpers
impl Pattern {
    fn new(data: Vec<Vec<char>>) -> Self {
        Pattern { data }
    }

    fn init() -> Self {
        let data = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];

        Pattern::new(data)
    }

    fn transpose(self) -> Self {
        let mut copy = self.clone();

        for y in 0..copy.data.len() {
            for x in 0..copy.data[y].len() {
                copy.data[x][y] = self.data[y][x];
            }
        }

        copy
    }

    fn flip(mut self) -> Self {
        self.data = self.data.into_iter().rev().collect::<Vec<_>>();
        self
    }

    fn permutate(self) -> Vec<Pattern> {
        let mut result = vec![self.clone()];

        let mut p = self.transpose();
        result.push(p.clone());
        p = p.flip();
        result.push(p.clone());
        p = p.transpose();
        result.push(p.clone());
        p = p.flip();
        result.push(p.clone());
        p = p.transpose();
        result.push(p.clone());
        p = p.flip();
        result.push(p.clone());
        p = p.transpose();
        result.push(p.clone());

        result
    }
}

impl PartialEq for Pattern {
    fn eq(&self, rhs: &Pattern) -> bool {
        if self.data.len() != rhs.data.len() {
            return false;
        }

        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] != rhs.data[y][x] {
                    return false;
                }
            }
        }

        true
    }
}

impl FromStr for Pattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let data = s.trim()
            .split("/")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Pattern::new(data))
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in &self.data {
            for c in y {
                f.write_char(*c)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Enhancement {
    matches: Vec<Pattern>,
    produces: Pattern,
}

impl FromStr for Enhancement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let rule = s
            .split(" => ")
            .map(|p| p.trim())
            .collect::<Vec<_>>();

        if rule.len() != 2 {
            Err(ParseError::new("Could not parse rule"))?;
        }

        let produces = Pattern::from_str(rule[1])?;

        let original = Pattern::from_str(rule[0])?;
        let matches = original.permutate();

        Ok(Enhancement { matches, produces })
    }
}

impl Enhancement {
    fn matches(&self, p: &Pattern) -> bool {
        self.matches.iter().filter(|m| *m == p).count() > 0
    }
}

#[aoc_generator(day21)]
fn get_input(input: &str) -> Result<Vec<Enhancement>, ParseError> {
    input
        .lines()
        .map(|l| Enhancement::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

// split and merge
impl Pattern {
    fn split(&self) -> Vec<Vec<Self>> {
        let step = if self.data.len() % 2 == 0 {
            2
        } else {
            3
        };

        let len = self.data.len();
        let mut result = vec![];
        for y in (0..len).step_by(step) {
            let mut row = vec![];
            for x in (0..len).step_by(step) {
                let mut tile = vec![];
                for i in 0..step {
                    let mut tile_row = vec![];
                    for j in 0..step {
                        tile_row.push(self.data[y + i][x + j]);
                    }
                    tile.push(tile_row);
                }
                row.push(Pattern::new(tile));
            }
            result.push(row);
        }

        result
    }

    fn merge(map: &Vec<Vec<Self>>) -> Self {
        let mut data = vec![];
        for y in 0..map.len() {
            let mut rows = vec![];
            for _ in 0..map[y][0].data.len() {
                rows.push(vec![]);
            }
            for x in 0..map[y].len() {
                for py in 0..map[y][x].data.len() {
                    for px in 0..map[y][x].data[py].len() {
                        rows[py].push(map[y][x].data[py][px]);
                    }
                }
            }

            for row in rows {
                data.push(row);
            }
        }

        Pattern::new(data)
    }

    fn enhance(self, enhancements: &Vec<Enhancement>) -> Result<Self, ParseError> {
        let mut split = self.split();

        for y in 0..split.len() {
            for x in 0..split[y].len() {
                let replacement = enhancements.iter()
                    .filter(|e| e.matches(&split[y][x]))
                    .map(|e| e.produces.clone())
                    .next()
                    .ok_or(ParseError::new("Could not find match"))?;

                split[y][x] = replacement;
            }
        }

        Ok(Pattern::merge(&split))
    }
}

#[aoc(day21, part1)]
fn problem1(input: &Vec<Enhancement>) -> Result<usize, ParseError> {
    let mut pattern = Pattern::init();

    for _ in 0..5 {
        pattern = pattern.enhance(input)?;
    }

    let result = pattern.data.iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum();

    Ok(result)
}

#[aoc(day21, part2)]
fn problem2(input: &Vec<Enhancement>) -> Result<usize, ParseError> {
    let mut pattern = Pattern::init();

    for _ in 0..18 {
        pattern = pattern.enhance(input)?;
    }

    let result = pattern.data.iter()
        .map(|r| r.iter().filter(|c| **c == '#').count())
        .sum();

    Ok(result)
}
