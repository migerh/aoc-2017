use std::collections::HashMap;
use regex::Regex;
use std::str::FromStr;
use crate::utils::ParseError;

#[derive(Debug)]
enum OperationType {
    Increase,
    Decrease,
}

#[derive(Debug)]
struct Operation {
    t: OperationType,
    register: String,
    value: isize,
}

impl Operation {
    fn new(t: OperationType, register: String, value: isize) -> Self {
        Operation { t, register, value }
    }

    fn apply(&self, registers: &mut HashMap<String, isize>) {
        let current_value = registers.get(&self.register).unwrap_or(&0);
        let new_value = match self.t {
            OperationType::Increase => current_value + self.value,
            OperationType::Decrease => current_value - self.value,
        };

        registers.entry(self.register.clone())
            .and_modify(|v| *v = new_value)
            .or_insert(new_value);
    }
}

#[derive(Debug)]
enum ConditionType {
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    Unequal,
}

#[derive(Debug)]
struct Condition {
    t: ConditionType,
    register: String,
    value: isize,
}

impl Condition {
    fn new(t: ConditionType, register: String, value: isize) -> Self {
        Condition { t, register, value }
    }

    fn eval(&self, registers: &HashMap<String, isize>) -> bool {
        let current_value = registers.get(&self.register).unwrap_or(&0);
        match self.t {
            ConditionType::Less => *current_value < self.value,
            ConditionType::LessOrEqual => *current_value <= self.value,
            ConditionType::Greater => *current_value > self.value,
            ConditionType::GreaterOrEqual => *current_value >= self.value,
            ConditionType::Equal => *current_value == self.value,
            ConditionType::Unequal => *current_value != self.value,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    cond: Condition,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(?P<opreg>.*)? (?P<op>.*)? (?P<opval>.*)? if (?P<conreg>.*)? (?P<cond>.*)? (?P<conval>.*)$").unwrap();
        }

        let (op, cond) = RE.captures(s).and_then(|cap| {
            let opreg = cap.name("opreg").map(|v| v.as_str())?.to_string();
            let op = cap.name("op").map(|v| v.as_str())?;
            let op = match op {
                "inc" => OperationType::Increase,
                "dec" => OperationType::Decrease,
                _ => None?,
            };
            let opval = cap.name("opval").map(|v| v.as_str().parse::<isize>())?.ok()?;

            let conreg = cap.name("conreg").map(|v| v.as_str())?.to_string();
            let cond = cap.name("cond").map(|v| v.as_str())?;
            let cond = match cond {
                "==" => ConditionType::Equal,
                "!=" => ConditionType::Unequal,
                "<=" => ConditionType::LessOrEqual,
                "<" => ConditionType::Less,
                ">=" => ConditionType::GreaterOrEqual,
                ">" => ConditionType::Greater,
                _ => None?,
            };
            let conval = cap.name("conval").map(|v| v.as_str().parse::<isize>())?.ok()?;

            let op = Operation::new(op, opreg, opval);
            let cond = Condition::new(cond, conreg, conval);
            Some((op, cond))
        }).ok_or(ParseError::new("Error during parse"))?;

        Ok(Self::new(op, cond))
    }
}

impl Instruction {
    fn new(op: Operation, cond: Condition) -> Self {
        Self { op, cond }
    }

    fn execute(&self, mut registers: &mut HashMap<String, isize>) {
        let op_register = self.op.register.clone();
        let cond_register = self.cond.register.clone();

        // Make sure registers are present in case all modified values
        // are below 0 in the end one of these could end up being the one
        // with the largest value.
        registers.entry(op_register).or_insert(0);
        registers.entry(cond_register).or_insert(0);

        if self.cond.eval(registers) {
            self.op.apply(&mut registers);
        }
    }
}

#[aoc_generator(day8)]
fn get_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines()
        .filter(|l| *l != "")
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn get_max(registers: &HashMap<String, isize>) -> Option<isize> {
    registers.iter()
        .map(|(_, v)| v)
        .cloned()
        .max()
}

#[aoc(day8, part1)]
fn problem1(input: &Vec<Instruction>) -> Result<isize, ParseError> {
    let mut registers = HashMap::new();

    for i in input {
        i.execute(&mut registers);
    }

    let max = get_max(&registers).ok_or(ParseError::new("Could not determine max value"))?;
    Ok(max)
}

#[aoc(day8, part2)]
fn problem2(input: &Vec<Instruction>) -> Result<isize, ParseError> {
    let mut registers = HashMap::new();
    let mut max = isize::MIN;

    for i in input {
        i.execute(&mut registers);
        let current_max = get_max(&registers).ok_or(ParseError::new("Could not determine max value"))?;
        max = std::cmp::max(max, current_max);
    }

    Ok(max)
}
