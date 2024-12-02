use regex::Regex;

pub fn numstring_to_numbers(input: &str) -> Vec<u64> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    re_num
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u64>().ok())
        .collect()
}

pub fn numbers_per_line(input: &str) -> Vec<Vec<u64>> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    input
        .lines()
        .map(|line| {
            re_num
                .captures_iter(line)
                .filter_map(|cap| cap[0].parse::<u64>().ok())
                .collect()
        })
        .collect()
}
