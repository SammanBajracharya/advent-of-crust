use std::fs::read_to_string;

pub fn part_1(file_path: String) -> i32 {
    let contents = read_to_string(file_path).unwrap();
    let mut result: i32 = 0;
    let chars: Vec<char> = contents.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if index >= 3 && chars[index] == '(' && &contents[index-3..index] == "mul" {
            if let Some(end) = (index..=(index+8).min(contents.len()))
                .find(|&i| chars[i] == ')')
            {
                let inner = &contents[index+1..end];
                if inner.contains(',') {
                    let mul: i32 = contents[index+1..end]
                        .split(',')
                        .filter_map(|s| s.trim().parse::<i32>().ok())
                        .product();
                    result += mul;
                }
            }
        }
        index += 1;
    }

    result
}
