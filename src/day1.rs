use std::iter::zip;

use regex::Regex;


fn numstring_to_numbers(input: &str) -> Vec<u64> {
    let re_num = Regex::new(r"[0-9]+").unwrap();
    re_num
        .captures_iter(input)
        .filter_map(|cap| cap[0].parse::<u64>().ok())
        .collect()
}

fn get_nth_row_numbers(n: usize, input: &str) -> Vec<u64> {
    numstring_to_numbers(input).into_iter().skip(n).step_by(2).collect()
}

fn get_first_row_numbers(input: &str) -> Vec<u64> {
    get_nth_row_numbers(0, input)
}

fn get_second_row_numbers(input: &str) -> Vec<u64> {
    get_nth_row_numbers(1, input)
}


pub fn task_one(input: String) -> u64 {
    let mut first_row_numbers = get_first_row_numbers(&input);
    let mut second_row_numbers = get_second_row_numbers(&input);

    first_row_numbers.sort();
    second_row_numbers.sort();

    zip(first_row_numbers.into_iter(), second_row_numbers.into_iter()).map(|(fst, snd)| fst.abs_diff(snd)).sum()
}

fn similarity_score(num: u64, others: &[u64]) -> u64 {
    let frequency = others.iter().filter(|o| **o == num).count() as u64;

    num * frequency
}

pub fn task_two(input: String) -> u64 {
    let first_row_numbers = get_first_row_numbers(&input);
    let second_row_numbers = get_second_row_numbers(&input);

    first_row_numbers.into_iter().map(|num| similarity_score(num, &second_row_numbers)).sum()
}
