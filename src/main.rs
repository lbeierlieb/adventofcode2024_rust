use std::fs::read_to_string;

mod helpers;

mod day1;
mod day2;
mod day3;

#[cfg(test)]
mod tests;

fn main() {
    println!(
        "1.1: {}",
        day1::task_one(read_to_string("inputs/day1").unwrap())
    );
    println!(
        "1.2: {}",
        day1::task_two(read_to_string("inputs/day1").unwrap())
    );
    println!(
        "2.1: {}",
        day2::task_one(read_to_string("inputs/day2").unwrap())
    );
    println!(
        "2.2: {}",
        day2::task_two(read_to_string("inputs/day2").unwrap())
    );
    println!(
        "3.1: {}",
        day3::task_one(read_to_string("inputs/day3").unwrap())
    );
    println!(
        "3.2: {}",
        day3::task_two(read_to_string("inputs/day3").unwrap())
    );
}
