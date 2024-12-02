use crate::Variation;

pub fn check_reports(reports: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for report in reports {
        if check_report(report) {
            sum += 1;
        }
    }
    sum
}

pub fn check_report(report: Vec<i32>) -> bool {
    let variation = Variation::from(report[0], report[1]);

    for level_number in 1..report.len() {
        let previous_level = report[level_number - 1];
        let level = report[level_number];

        let difference = (previous_level - level).abs();

        if difference < 1 || 3 < difference {
            return false;
        }

        match variation {
            Variation::Increase => {
                if previous_level > level {
                    return false;
                }
            }
            Variation::Decrease => {
                if previous_level < level {
                    return false;
                }
            }
        }
    }

    true
}
