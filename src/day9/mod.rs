use std::collections::HashMap;

#[aoc_generator(day9)]
fn get_input(input: &str) -> Vec<char> {
    input.trim().chars().collect::<Vec<_>>()
}

enum ParserState {
    Read,
    Garbage,
}

#[aoc(day9, part1)]
fn problem1(input: &Vec<char>) -> usize {
    let mut it = input.iter();
    let mut histogram = HashMap::new();
    let mut level = 0;
    let mut state = ParserState::Read;

    while let Some(c) = it.next() {
        match (&state, c) {
            (ParserState::Read, '{') => {
                level += 1;
            },
            (ParserState::Read, '}') => {
                histogram.entry(level).and_modify(|v| *v += 1).or_insert(1);
                level -= 1;
            },
            (ParserState::Read, '<') => {
                state = ParserState::Garbage;
            },
            (ParserState::Garbage, '>') => {
                state = ParserState::Read;
            },
            (_, '!') => {
                it.next();
            },
            (_, _) => {},
        }
    }

    let score = histogram.iter().map(|(k, v)| k * v).sum();

    score
}

#[aoc(day9, part2)]
fn problem2(input: &Vec<char>) -> usize {
    let mut it = input.iter();
    let mut state = ParserState::Read;
    let mut sum = 0;

    while let Some(c) = it.next() {
        match (&state, c) {
            (ParserState::Read, '<') => {
                state = ParserState::Garbage;
            },
            (ParserState::Garbage, '>') => {
                state = ParserState::Read;
            },
            (_, '!') => {
                it.next();
            },
            (ParserState::Garbage, _) => {
                sum += 1;
            },
            (_, _) => {},
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let input = get_input("{}");
        assert_eq!(1, problem1(&input));
    }

    #[test]
    pub fn example_1_2() {
        let input = get_input("{{{}}}");
        assert_eq!(6, problem1(&input));
    }

    #[test]
    pub fn example_1_3() {
        let input = get_input("{{},{}}");
        assert_eq!(5, problem1(&input));
    }

    #[test]
    pub fn example_1_4() {
        let input = get_input("{{{},{},{{}}}}");
        assert_eq!(16, problem1(&input));
    }

    #[test]
    pub fn example_1_5() {
        let input = get_input("{<a>,<a>,<a>,<a>}");
        assert_eq!(1, problem1(&input));
    }

    #[test]
    pub fn example_1_6() {
        let input = get_input("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        assert_eq!(9, problem1(&input));
    }

    #[test]
    pub fn example_1_7() {
        let input = get_input("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        assert_eq!(9, problem1(&input));
    }

    #[test]
    pub fn example_1_8() {
        let input = get_input("{{<a!>},{<a!>},{<a!>},{<ab>}}");
        assert_eq!(3, problem1(&input));
    }

    #[test]
    pub fn example_2_1() {
        let input = get_input("<>");
        assert_eq!(0, problem2(&input));
    }

    #[test]
    pub fn example_2_2() {
        let input = get_input("<random characters>");
        assert_eq!(17, problem2(&input));
    }

    #[test]
    pub fn example_2_3() {
        let input = get_input("<<<<>");
        assert_eq!(3, problem2(&input));
    }

    #[test]
    pub fn example_2_4() {
        let input = get_input("<{!>}>");
        assert_eq!(2, problem2(&input));
    }

    #[test]
    pub fn example_2_5() {
        let input = get_input("<!!>");
        assert_eq!(0, problem2(&input));
    }

    #[test]
    pub fn example_2_6() {
        let input = get_input("<!!!>>");
        assert_eq!(0, problem2(&input));
    }

    #[test]
    pub fn example_2_7() {
        let input = get_input("<{o\"i!a,<{i<a>");
        assert_eq!(10, problem2(&input));
    }
}
