use std::fs::read_to_string;

pub fn part_2(file_path: String) {
    let contents = read_to_string(file_path).expect("File not found");

    let rules: Vec<(usize, usize)> = contents
        .lines()
        .filter(|line| line.contains("|") && !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split("|").collect();
            (parts[0].parse().unwrap(), parts[1].parse().unwrap())
        })
        .collect();

    let mut updates: Vec<Vec<usize>> = contents
        .lines()
        .filter(|line| !line.contains("|") && !line.is_empty())
        .map(|line| {
            line.split(",")
                .filter_map(|page| page.trim().parse().ok())
                .collect()
        })
        .collect();

    let mut valid_updates: Vec<Vec<usize>> = vec![];

    for update in &mut updates {
        let mut valid: bool = true;
        for (i, page) in update.iter().enumerate() {
            let valid_rules: Vec<_> = rules
                .iter()
                .filter(|rule| rule.0 == *page)
                .collect();
            for valid_rule in &valid_rules {
                if let Some(index) = update.iter().position(|&x| x == valid_rule.1) {
                    if i > index {
                        valid = false;
                        break;
                    }
                }
            }
        }
        if !valid {
            valid_updates.push(update.to_vec());
        }
    }

    for update in &mut valid_updates {
        let mut i = 0;
        while i < update.len() {
            let page = update[i];

            for valid_rule in rules.iter().filter(|rule| rule.0 == page) {
                if let Some(index) = update.iter().position(|&x| x == valid_rule.1) {
                    if i > index {
                        update.swap(i, index);
                        i = index;
                        continue;
                    }
                }
            }
            i+=1;
        }
    }

    let mut sum = 0;
    for update in valid_updates {
        sum += update[update.len()/2];
    }
    println!("{}", sum);
}
