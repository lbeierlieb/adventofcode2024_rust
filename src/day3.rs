use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    MultiplyPair(u64, u64),
}

#[derive(Debug)]
struct State {
    enabled: bool,
    sum: u64,
}

impl State {
    fn initial() -> State {
        State {
            enabled: true,
            sum: 0,
        }
    }

    fn set_enabled(&self, val: bool) -> State {
        State {
            enabled: val,
            sum: self.sum,
        }
    }

    fn add_to_sum(&self, val: u64) -> State {
        let add = if self.enabled { val } else { 0 };
        State {
            enabled: self.enabled,
            sum: self.sum + add,
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    let re_num = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re_num
        .captures_iter(input)
        .map(|cap| {
            if cap[0].starts_with("don") {
                Instruction::Dont
            } else if cap[0].starts_with("do") {
                Instruction::Do
            } else {
                let a = cap[1].parse::<u64>().unwrap();
                let b = cap[2].parse::<u64>().unwrap();
                Instruction::MultiplyPair(a, b)
            }
        })
        .collect()
}

pub fn task_one(input: String) -> u64 {
    parse(&input)
        .into_iter()
        .map(|ins| match ins {
            Instruction::MultiplyPair(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

pub fn task_two(input: String) -> u64 {
    let initial_state = State::initial();
    parse(&input)
        .into_iter()
        .fold(initial_state, |state, ins| match ins {
            Instruction::Do => state.set_enabled(true),
            Instruction::Dont => state.set_enabled(false),
            Instruction::MultiplyPair(a, b) => state.add_to_sum(a * b),
        })
        .sum
}
