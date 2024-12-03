use std::fs::read_to_string;

use regex::Regex;

#[derive(PartialEq)]
enum Instruction {
    Do,
    Dont,
}
fn main() {
    let lines: Vec<String> = read_to_string("day3/src/input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let cleared_program = extract_data(lines.clone());
    let cleared_program_with_condition = extract_data_with_condition(lines);

    println! {"{:?}", execute_cleared_program(cleared_program)};
    println! {"{:?}", execute_cleared_program(cleared_program_with_condition)};
}

fn extract_data(lines: Vec<String>) -> Vec<(i32, i32)> {
    let mul_regex = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    let mut multiplication_numbers = vec![];

    for line in lines {
        for caps in mul_regex.captures_iter(&line) {
            let first_number: i32 = caps["first"].parse().unwrap();
            let second_number: i32 = caps["second"].parse().unwrap();
            multiplication_numbers.push((first_number, second_number));
        }
    }

    multiplication_numbers
}

fn extract_data_with_condition(lines: Vec<String>) -> Vec<(i32, i32)> {
    let mul_regex = Regex::new(r"(do\(\)|don't\(\)|mul\((?<first>\d+),(?<second>\d+)\))").unwrap();
    let mut multiplication_numbers = vec![];
    let mut instruction = Instruction::Do;

    for line in lines {
        for caps in mul_regex.captures_iter(&line) {
            match &caps[0] {
                "do()" => instruction = Instruction::Do,
                "don't()" => instruction = Instruction::Dont,
                _ => {
                    if instruction == Instruction::Do {
                        let first_number: i32 = caps["first"].parse().unwrap();
                        let second_number: i32 = caps["second"].parse().unwrap();
                        multiplication_numbers.push((first_number, second_number));
                    }
                }
            }
        }
    }

    multiplication_numbers
}

fn execute_cleared_program(numbers: Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for (first_num, second_num) in numbers {
        sum += first_num * second_num;
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::{extract_data, extract_data_with_condition};

    #[test]
    fn should_extract_data() {
        assert_eq!(
            vec![(17, 938), (3, 2)],
            extract_data(vec![String::from(
                "aaamul(17,938)_dkjfb,mul(13d34)sdfmul(3,2)"
            )])
        );
    }

    #[test]
    fn should_extract_only_do_data() {
        assert_eq!(
            vec![(17, 938), (1, 2)],
            extract_data_with_condition(vec![String::from(
                "aaamul(17,938)_dkjfbdon't(),mul(13d34)sdfmul(3,2)do()mul(1,2)"
            )])
        );
    }
}
