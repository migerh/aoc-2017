use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::ParseError;

fn first(s: &str) -> Result<char, ParseError> {
    s.chars().next().ok_or(ParseError::new("Cannot parse empty string"))
}

#[derive(Debug, Clone)]
enum Param {
    Register(char),
    Value(isize),
}

impl FromStr for Param {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        use Param::*;

        let first = first(s)?;
        if s.len() == 1 && first.is_alphabetic() {
            return Ok(Register(first));
        }

        Ok(Value(s.parse::<isize>()?))
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Set((char, Param)),
    Sub((char, Param)),
    Mul((char, Param)),
    Jnz((Param, Param)),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        use Instruction::*;

        let i = s.split(" ").collect::<Vec<_>>();

        if i.len() < 2 || i.len() > 3 {
            return Err(ParseError::new("Invalid input"));
        }

        Ok(match i[0] {
            "set" => Set((first(i[1])?, Param::from_str(i[2])?)),
            "sub" => Sub((first(i[1])?, Param::from_str(i[2])?)),
            "mul" => Mul((first(i[1])?, Param::from_str(i[2])?)),
            "jnz" => Jnz((Param::from_str(i[1])?, Param::from_str(i[2])?)),
            _ => Err(ParseError::new("Invalid instruction"))?
        })
    }
}

#[derive(Debug)]
struct Duet {
    instructions: Vec<Instruction>,
    registers: HashMap<char, isize>,
    muls: usize,
}

impl Duet {
    fn new(instructions: &Vec<Instruction>) -> Duet {
        let instructions = instructions.clone();
        let registers = HashMap::new();
        let muls = 0;

        Duet { instructions, registers, muls }
    }

    fn resolve(&self, p: Param) -> isize {
        use Param::*;

        match p {
            Register(r) => *self.registers.get(&r).unwrap_or(&0),
            Value(v) => v,
        }
    }

    fn set(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r = v).or_insert(v);
    }

    fn sub(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r -= v).or_insert(v);
    }

    fn mul(&mut self, a: char, p: Param) {
        self.muls += 1;

        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r *= v).or_insert(0);
    }

    fn jnz(&self, a: Param, b: Param) -> isize {
        let v = self.resolve(a);

        if v != 0 {
            self.resolve(b)
        } else {
            1
        }
    }

    fn run(&mut self) {
        use Instruction::*;

        let mut counter = 0;
        let mut i = 0;
        let instructions = self.instructions.clone();
        while i < self.instructions.len() {
            let instr = &instructions[i];

            match instr {
                Set((a, p)) => self.set(*a, p.clone()),
                Sub((a, p)) => self.sub(*a, p.clone()),
                Mul((a, p)) => self.mul(*a, p.clone()),
                Jnz((a, p)) => i = ((i as isize) + self.jnz(a.clone(), p.clone()) - 1) as usize,
            }

            i += 1;
            counter += 1;

            if counter > 100_000 {
                break;
            }
        }
    }
}

#[aoc_generator(day23)]
fn get_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[aoc(day23, part1)]
fn problem1(input: &Vec<Instruction>) -> Result<usize, ParseError> {
    let mut cocpu = Duet::new(input);

    cocpu.run();

    Ok(cocpu.muls)
}

// refactor the assembly code into rust code and optimize it
fn count_nonprimes(start: usize, end: usize) -> usize {
    let mut h = 0;
    for b in (start..=end).step_by(17) {

        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
    }

    h
}

// guesses: 1001
#[aoc(day23, part2)]
fn problem2(_input: &Vec<Instruction>) -> Result<usize, ParseError> {
    let result = count_nonprimes(106500, 123500);
    Ok(result)
}
