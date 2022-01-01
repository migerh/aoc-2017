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
    Snd(char),
    Set((char, Param)),
    Add((char, Param)),
    Mul((char, Param)),
    Mod((char, Param)),
    Rcv(char),
    Jgz((Param, Param)),
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
            "snd" => Snd(first(i[1])?),
            "set" => Set((first(i[1])?, Param::from_str(i[2])?)),
            "add" => Add((first(i[1])?, Param::from_str(i[2])?)),
            "mul" => Mul((first(i[1])?, Param::from_str(i[2])?)),
            "mod" => Mod((first(i[1])?, Param::from_str(i[2])?)),
            "rcv" => Rcv(first(i[1])?),
            "jgz" => Jgz((Param::from_str(i[1])?, Param::from_str(i[2])?)),
            _ => Err(ParseError::new("Invalid instruction"))?
        })
    }
}

#[aoc_generator(day18)]
fn get_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

#[derive(Debug)]
struct Duet {
    instructions: Vec<Instruction>,
    registers: HashMap<char, isize>,
    frequency: Option<isize>,
    v2: bool,
    machine: isize,
    sent: usize,
    ip: usize,
}

impl Duet {
    fn new(instructions: &Vec<Instruction>) -> Duet {
        let instructions = instructions.clone();
        let registers = HashMap::new();
        let frequency = None;
        let sent = 0;
        let machine = 0;
        let ip = 0;

        Duet { instructions, registers, frequency, v2: false, machine, sent, ip }
    }

    fn v2(instructions: &Vec<Instruction>, machine: isize) -> Self {
        let mut duet = Duet::new(instructions);

        duet.machine = machine;
        duet.v2 = true;
        duet.registers.entry('p').or_insert(machine);

        duet
    }

    fn resolve(&self, p: Param) -> isize {
        use Param::*;

        match p {
            Register(r) => *self.registers.get(&r).unwrap_or(&0),
            Value(v) => v,
        }
    }

    fn snd(&mut self, register: char) {
        let value = self.registers.get(&register).unwrap_or(&0);

        if self.v2 {
            return;
        }

        self.frequency = Some(*value);
    }

    fn set(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r = v).or_insert(v);
    }

    fn add(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r += v).or_insert(v);
    }

    fn mul(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r *= v).or_insert(0);
    }

    fn mod_(&mut self, a: char, p: Param) {
        let v = self.resolve(p);
        self.registers.entry(a).and_modify(|r| *r = *r % v).or_insert(0);
    }

    fn rcv(&mut self, a: char) {
        if self.v2 {
            return
        }

        let v = *self.registers.get(&a).unwrap_or(&0);

        if v == 0 {
            return;
        }

        if let Some(f) = self.frequency {
            self.registers.entry(a).and_modify(|r| *r = f);
        }
    }

    fn jgz(&self, a: Param, b: Param) -> isize {
        let v = self.resolve(a);

        if v > 0 {
            self.resolve(b)
        } else {
            1
        }
    }

    fn run(&mut self) -> Option<isize> {
        use Instruction::*;

        let mut i = 0;
        let instructions = self.instructions.clone();
        while i < self.instructions.len() {
            let instr = &instructions[i];

            if !self.v2 {
                if let Rcv(r) = instr {
                    let v = *self.registers.get(&r).unwrap_or(&0);
                    if v != 0 && self.frequency.is_some() {
                        break;
                    }
                }
            }

            match instr {
                Snd(f) => self.snd(*f),
                Set((a, p)) => self.set(*a, p.clone()),
                Add((a, p)) => self.add(*a, p.clone()),
                Mul((a, p)) => self.mul(*a, p.clone()),
                Mod((a, p)) => self.mod_(*a, p.clone()),
                Rcv(a) => self.rcv(*a),
                Jgz((a, p)) => i = ((i as isize) + self.jgz(a.clone(), p.clone()) - 1) as usize,
            }

            i += 1;
        }

        self.frequency
    }

    fn step(&mut self, buffer_in: Vec<isize>) -> Vec<isize> {
        use Instruction::*;

        let mut buffer_in = buffer_in.into_iter().rev().collect::<Vec<_>>();
        let mut buffer_out = Vec::with_capacity(500_000_000);
        let mut i = self.ip;
        let instructions = self.instructions.clone();
        while i < self.instructions.len() {
            let instr = &instructions[i];

            match instr {
                Snd(f) => {
                    self.sent += 1;
                    let v = self.registers.get(f).unwrap_or(&0);
                    buffer_out.push(*v);
                },
                Set((a, p)) => self.set(*a, p.clone()),
                Add((a, p)) => self.add(*a, p.clone()),
                Mul((a, p)) => self.mul(*a, p.clone()),
                Mod((a, p)) => self.mod_(*a, p.clone()),
                Rcv(a) => {
                    if let Some(value) = buffer_in.pop() {
                        self.registers.entry(*a).and_modify(|v| *v = value).or_insert(value);
                    } else {
                        self.ip = i;
                        return buffer_out;
                    }
                }
                Jgz((a, b)) => i = ((i as isize) + self.jgz(a.clone(), b.clone()) - 1) as usize,
            }

            i += 1;
        }

        self.ip = i;
        buffer_out
    }
}

#[aoc(day18, part1)]
fn problem1(input: &Vec<Instruction>) -> Result<isize, ParseError> {
    let mut duet = Duet::new(input);
    duet.run().ok_or(ParseError::new("Could not find frequency"))
}

#[aoc(day18, part2)]
fn problem2(input: &Vec<Instruction>) -> Result<usize, ParseError> {
    let mut duets = [Duet::v2(input, 0), Duet::v2(input, 1)];
    let mut current = 0;
    let mut buffer = vec![];
    let mut last_out = [10, 10];

    loop {
        buffer = duets[current].step(buffer);

        last_out[current] = buffer.len();
        current = (current + 1) % 2;

        if last_out[0] == 0 && last_out[1] == 0 {
            break;
        }
    }

    Ok(duets[1].sent)
}
