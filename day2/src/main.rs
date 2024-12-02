use std::fs::read_to_string;

use part_one::{check_report, check_reports};

mod part_one;
#[derive(Debug, Clone, Copy)]
enum Variation {
    Increase,
    Decrease,
}

fn main() {
    let lines: Vec<String> = read_to_string("day2/src/input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let reports = extract_data(lines);

    println! {"{:?}", check_reports(reports.clone())}
    println! {"{:?}", check_reports_dampener(reports)}
}

fn extract_data(lines: Vec<String>) -> Vec<Vec<i32>> {
    let mut reports = vec![];

    for line in lines {
        let mut numbers = line.split_ascii_whitespace();

        let mut levels = vec![];
        while let Some(level) = numbers.next() {
            levels.push(level.parse::<i32>().unwrap());
        }
        reports.push(levels);
    }

    reports
}

pub fn check_reports_dampener(reports: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for report in reports {
        if check_report_dampener(report) {
            sum += 1;
        }
    }
    sum
}

fn check_report_dampener(report: Vec<i32>) -> bool {
    for level_number in 0..report.len() {
        let mut possible_solution = report.clone();
        possible_solution.remove(level_number);
        if check_report(possible_solution) {
            return true;
        }
    }

    false
}

impl Variation {
    fn from(level_1: i32, level_2: i32) -> Variation {
        if level_1 > level_2 {
            Variation::Decrease
        } else {
            Variation::Increase
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{check_report_dampener, part_one::check_report};

    #[test]
    fn should_be_safe_when_decreasing() {
        assert!(check_report(vec![7, 6, 4, 2, 1]));
    }

    #[test]
    fn should_be_safe_when_increasing() {
        assert!(check_report(vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn should_be_unsafe_when_changing_value_too_much() {
        assert!(!check_report(vec![1, 2, 7, 8, 9]));
        assert!(!check_report(vec![9, 7, 6, 2, 1]));
    }

    #[test]
    fn should_be_unsafe_when_level_does_not_change() {
        assert!(!check_report(vec![8, 6, 4, 4, 1]));
    }

    #[test]
    fn should_be_safe_when_possibly_removing_one_level() {
        assert!(check_report_dampener(vec![7, 6, 4, 2, 1]));
        assert!(check_report_dampener(vec![1, 3, 6, 7, 9]));
        assert!(check_report_dampener(vec![1, 3, 2, 4, 5]));
        assert!(check_report_dampener(vec![8, 6, 4, 4, 1]));
        assert!(!check_report_dampener(vec![1, 2, 7, 8, 9]));
        assert!(!check_report_dampener(vec![9, 7, 6, 2, 1]));
        assert!(check_report_dampener(vec![1, 4, 2, 3, 4]));
        assert!(check_report_dampener(vec![4, 3, 2, 4, 1]));
    }
}
