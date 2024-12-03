use regex::Regex;

enum Instruction {
    Do,
    Dont,
    MultiplyPair(u64, u64),
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
    0
}
