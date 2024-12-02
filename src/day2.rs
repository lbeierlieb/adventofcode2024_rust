use crate::helpers::parsing::numbers_per_line;

fn check_diffs<F>(report: &[u64], check_function: F) -> bool
where
    F: Fn(i64) -> bool,
{
    report
        .windows(2)
        .map(|window| window[1] as i64 - window[0] as i64)
        .all(check_function)
}

fn is_report_valid(report: &[u64]) -> bool {
    let all_increasing = check_diffs(report, |diff| diff > 0);
    let all_decreasing = check_diffs(report, |diff| diff < 0);
    let small_steps = check_diffs(report, |diff| diff.abs() >= 1 && diff.abs() <= 3);

    (all_increasing || all_decreasing) && small_steps
}

fn is_report_valid_one_removed(report: &[u64]) -> bool {
    for i in 0..report.len() {
        let mut shortened_report = report.to_vec();
        shortened_report.remove(i);
        if is_report_valid(&shortened_report) {
            return true;
        }
    }
    false
}

fn process<F>(input: &str, report_checking_function: F) -> u64
where
    F: Fn(&[u64]) -> bool,
{
    let reports = numbers_per_line(input);
    reports
        .into_iter()
        .filter(|report| report_checking_function(report))
        .count() as u64
}

pub fn task_one(input: String) -> u64 {
    process(&input, is_report_valid)
}

pub fn task_two(input: String) -> u64 {
    process(&input, is_report_valid_one_removed)
}
