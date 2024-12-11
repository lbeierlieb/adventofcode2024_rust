use std::fs::read_to_string;

#[test]
fn day1_task1() {
    let input = read_to_string("inputs/day1_task1_test").unwrap();
    let result = 11;
    assert_eq!(result, crate::day1::task_one(input));
}

#[test]
fn day1_task2() {
    let input = read_to_string("inputs/day1_task2_test").unwrap();
    let result = 31;
    assert_eq!(result, crate::day1::task_two(input));
}

#[test]
fn day2_task1() {
    let input = read_to_string("inputs/day2_task1_test").unwrap();
    let result = 2;
    assert_eq!(result, crate::day2::task_one(input));
}

#[test]
fn day2_task2() {
    let input = read_to_string("inputs/day2_task2_test").unwrap();
    let result = 4;
    assert_eq!(result, crate::day2::task_two(input));
}

#[test]
fn day3_task1() {
    let input = read_to_string("inputs/day3_task1_test").unwrap();
    let result = 161;
    assert_eq!(result, crate::day3::task_one(input));
}

#[test]
fn day3_task2() {
    let input = read_to_string("inputs/day3_task2_test").unwrap();
    let result = 48;
    assert_eq!(result, crate::day3::task_two(input));
}

#[test]
fn day4_task1() {
    let input = read_to_string("inputs/day4_task1_test").unwrap();
    let result = 18;
    assert_eq!(result, crate::day4::task_one(input));
}

#[test]
fn day4_task2() {
    let input = read_to_string("inputs/day4_task2_test").unwrap();
    let result = 9;
    assert_eq!(result, crate::day4::task_two(input));
}

#[test]
fn day5_task1() {
    let input = read_to_string("inputs/day5_task1_test").unwrap();
    let result = 143;
    assert_eq!(result, crate::day5::task_one(input));
}

#[test]
fn day5_task2() {
    let input = read_to_string("inputs/day5_task2_test").unwrap();
    let result = 123;
    assert_eq!(result, crate::day5::task_two(input));
}

#[test]
fn day6_task1() {
    let input = read_to_string("inputs/day6_task1_test").unwrap();
    let result = 41;
    assert_eq!(result, crate::day6::task_one(input));
}

#[test]
fn day6_task2() {
    let input = read_to_string("inputs/day6_task2_test").unwrap();
    let result = 6;
    assert_eq!(result, crate::day6::task_two(input));
}

#[test]
fn day7_task1() {
    let input = read_to_string("inputs/day7_task1_test").unwrap();
    let result = 3749;
    assert_eq!(result, crate::day7::task_one(input));
}

#[test]
fn day7_task2() {
    let input = read_to_string("inputs/day7_task2_test").unwrap();
    let result = 11387;
    assert_eq!(result, crate::day7::task_two(input));
}

#[test]
fn day8_task1() {
    let input = read_to_string("inputs/day8_task1_test").unwrap();
    let result = 14;
    assert_eq!(result, crate::day8::task_one(input));
}

#[test]
fn day8_task2() {
    let input = read_to_string("inputs/day8_task2_test").unwrap();
    let result = 34;
    assert_eq!(result, crate::day8::task_two(input));
}

#[test]
fn day9_task1() {
    let input = read_to_string("inputs/day9_task1_test").unwrap();
    let result = 1928;
    assert_eq!(result, crate::day9::task_one(input));
}

#[test]
fn day9_task2() {
    let input = read_to_string("inputs/day9_task2_test").unwrap();
    let result = 2858;
    assert_eq!(result, crate::day9::task_two(input));
}

#[test]
fn day10_task1() {
    let input = read_to_string("inputs/day10_task1_test").unwrap();
    let result = 36;
    assert_eq!(result, crate::day10::task_one(input));
}

#[test]
fn day10_task2() {
    let input = read_to_string("inputs/day10_task2_test").unwrap();
    let result = 81;
    assert_eq!(result, crate::day10::task_two(input));
}

#[test]
fn day11_task1() {
    let input = read_to_string("inputs/day11_task1_test").unwrap();
    let result = 55312;
    assert_eq!(result, crate::day11::task_one(input));
}
