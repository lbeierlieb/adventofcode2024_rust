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
