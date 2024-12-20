use std::fs::read_to_string;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_2(file_path: String) -> i32 {
    let mut left_arr: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();
    let contents = read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let left_num = parts[0].parse::<i32>();
        let right_num = parts[1].parse::<i32>();
        if let (Ok(left), Ok(right)) = (left_num, right_num) {
            left_arr.push(left);
            *right_map.entry(right).or_insert(0) += 1;
        }
    }

    let mut sum: i32 = 0;
    for &num in &left_arr {
        let count = *right_map.get(&num).unwrap_or(&0);
        sum += num * count;
    }

    sum
}

