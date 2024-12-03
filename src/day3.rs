use regex::Regex;

fn parse(input: &str) -> Vec<(u64, u64)> {
    let re_num = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re_num
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<u64>().unwrap(),
                cap[2].parse::<u64>().unwrap(),
            )
        })
        .collect()
}

pub fn task_one(input: String) -> u64 {
    parse(&input).into_iter().map(|(a, b)| a * b).sum()
}

pub fn task_two(input: String) -> u64 {
    0
}
