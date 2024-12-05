use std::fs::read_to_string;

pub fn part_2(file_path: String) -> i32 {
    let contents = read_to_string(file_path).expect("File not found");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut count = 0;
    let directions: Vec<(isize, isize)> = vec![
        (0, 0),     // current
        (1, 1),     // down-right
        (-1, -1),   // up-left
    ];

    let directions_op: Vec<(isize, isize)> = vec![
        (0, 0),     // current
        (1, -1),    // down-left
        (-1, 1),    // up-right
    ];

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row].chars().nth(col) == Some('A') {
                let mut temp = String::new();
                let mut r = row as isize;
                let mut c = col as isize;

                for direction in &directions {
                    r += direction.0;
                    c += direction.1;
                    if r < 0 || r >= lines.len() as isize || c < 0 || c >= lines[r as usize].len() as isize {
                        break;
                    }

                    temp.push(lines[r as usize].chars().nth(c as usize).unwrap());

                    r = row as isize;
                    c = col as isize;
                }

                let mut temp: Vec<char> = temp.chars().collect();
                temp.sort(); // Should be ['A', 'M', 'S']

                if temp != ['A', 'M', 'S'] { continue; }

                let mut temp_op = String::new();
                r = row as isize;
                c = col as isize;

                for direction in &directions_op {
                    r += direction.0;
                    c += direction.1;
                    if r < 0 || r >= lines.len() as isize || c < 0 || c >= lines[r as usize].len() as isize {
                        break;
                    }

                    temp_op.push(lines[r as usize].chars().nth(c as usize).unwrap());

                    r = row as isize;
                    c = col as isize;
                }

                let mut temp_op: Vec<char> = temp_op.chars().collect();
                temp_op.sort();

                if temp_op == ['A', 'M', 'S'] { count += 1; }
            }
        }
    }

    count
}

