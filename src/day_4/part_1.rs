use std::fs::read_to_string;

pub fn part_1(file_path: String) -> i32 {
    let contents = read_to_string(file_path).expect("File not found");
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let mut count = 0;
    let directions: Vec<(isize, isize)> = vec![
        (0, 1),  // right
        (0, -1), // left
        (1, 0),  // down
        (-1, 0), // up
        (1, 1),  // down-right
        (1, -1), // down-left
        (-1, 1), // up-right
        (-1, -1), // up-left
    ];

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row].chars().nth(col) == Some('X') {
                for direction in &directions {
                    let mut temp = String::new();
                    let mut r = row as isize;
                    let mut c = col as isize;

                    for _ in 0..4 {
                        if r < 0 || r >= lines.len() as isize || c < 0 || c >= lines[r as usize].len() as isize {
                            break;
                        }

                        temp.push(lines[r as usize].chars().nth(c as usize).unwrap());

                        r += direction.0;
                        c += direction.1;
                    }

                    if temp == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
