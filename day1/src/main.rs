use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let lines: Vec<String> = read_to_string("day1/src/input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let (list_1, list_2) = extract_data(lines);

    println!(
        "distance : {:?}",
        compute_list_distance(list_1.clone(), list_2.clone())
    );
    println!(
        "similarity : {:?}",
        compute_similarity_score(list_1, list_2)
    );
}

fn extract_data(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = vec![];
    let mut second_list = vec![];

    for line in lines {
        let mut numbers = line.split_ascii_whitespace();

        first_list.push(numbers.next().unwrap().parse().unwrap());
        second_list.push(numbers.next().unwrap().parse().unwrap());
    }

    (first_list, second_list)
}

fn compute_list_distance(mut list_1: Vec<i32>, mut list_2: Vec<i32>) -> i32 {
    list_1.sort();
    list_2.sort();

    let mut sum = 0;

    for (num_1, num_2) in list_1.iter().zip(list_2.iter()) {
        sum += (num_1 - num_2).abs();
    }

    sum
}

fn compute_similarity_score(list_1: Vec<i32>, list_2: Vec<i32>) -> i32 {
    let mut list_2_summary = HashMap::new();
    for element in list_2 {
        list_2_summary
            .entry(element)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut sum = 0;

    for num in list_1 {
        sum += num * list_2_summary.get(&num).unwrap_or(&0);
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::{compute_list_distance, compute_similarity_score};

    #[test]
    fn should_compute_list_distance() {
        let list_1 = vec![3, 4, 2, 1, 3, 3];
        let list_2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(11, compute_list_distance(list_1, list_2));
    }

    #[test]
    fn should_compute_similarity_score() {
        let list_1 = vec![3, 4, 2, 1, 3, 3];
        let list_2 = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(31, compute_similarity_score(list_1, list_2));
    }
}
