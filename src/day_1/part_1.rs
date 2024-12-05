use std::fs::read_to_string;
use crate::day_1::quicksort::quicksort;

#[allow(dead_code)]
pub fn part_1(file_path: String) -> i32 {
    let mut left_arr: Vec<i32> = vec![];
    let mut right_arr: Vec<i32> = vec![];
    let contents = read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let left_num = parts[0].parse::<i32>();
        let right_num = parts[1].parse::<i32>();
        if let (Ok(left), Ok(right)) = (left_num, right_num) {
            left_arr.push(left);
            right_arr.push(right);
        }
    }

    quicksort(&mut left_arr);
    quicksort(&mut right_arr);

    let mut sum: i32 = 0;
    for i in 0..left_arr.len() {
        sum += (left_arr[i] - right_arr[i]).abs();
    }

    sum
}

