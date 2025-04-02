use std::fs::read_to_string;

pub fn part_1(file_path: String) {
    let contents = read_to_string(file_path).expect("File not found");
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|x| x.chars().collect())
        .collect();


    let (mut x_pos, mut y_pos): (usize, usize) = (0, 0);

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] == '^' {
                x_pos = x;
                y_pos = y;
            }
        }
    }

    let path_dir: [(isize, isize); 4] = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ];

    let mut i = 0;

    while x_pos < map.len() && y_pos < map[0].len() {
        let new_x = x_pos as isize + path_dir[i].0;
        let new_y = y_pos as isize + path_dir[i].1;
        if new_x >= 0 && new_y >= 0 {
            if let Some(&cell) = map.get(new_x as usize).and_then(|row| row.get(new_y as usize)) {
                if cell == '#' {
                    i = (i + 1) % 4;
                } else {
                    map[x_pos][y_pos] = 'X';
                    x_pos = new_x as usize;
                    y_pos = new_y as usize;
                }
            } else { break; }
        } else { break; }

        map[x_pos][y_pos] = 'X'; // Mark visited position
    }

    let count = map.iter()
        .flat_map(|row| row.iter())
        .filter(|&&ch| ch == 'X')
        .count();

    println!("{}", count);
}
