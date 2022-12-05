fn parse_inital_condition_row(line: &str) -> Vec<Option<char>> {
    let line: Vec<char> = line.chars().collect();
    let mut rem = &line[..];
    let mut acc = vec![];
    while rem.len() > 0 {
        let mut record = &rem[..3];
        if rem.len() == 3 {
            record = &rem[..3];
            rem = &rem[3..]
        } else {
            record = &rem[..4];
            rem = &rem[4..];
        }
        match &record[1] {
            ' ' => acc.push(None),
            a => acc.push(Some(*a))
        }
    }
    acc
}

fn main() {
    // Parser code, do not touch.
    // Ok, I will comment it
    // Container to hold data as it is save in the file [row (inverted)][col]
    let mut inital_condition_raw: Vec<Vec<Option<char>>> = Vec::new();
    let mut lines = std::io::stdin().lines();
    // For all lines until a blank line, cal prase_inital_conditons_row to parse the line and place
    // the result into inital_conditons_raw
    loop {
        let line = lines.next().expect("EOF parsing inital state").expect("IO error parsing inital state");
        if line.len() == 0 {
            break;
        }
        inital_condition_raw.push(parse_inital_condition_row(&line));
    }
    // Determine the dimentiosn of the inital conditons, acounting for the "1 2 3 4..." row at the
    // bottom
    let columns = inital_condition_raw[0].len();
    let rows = inital_condition_raw.len() - 1;
    // Reorder the data into a sane arangemnt: [col][row], with no None values
    let mut inital_conditons: Vec<Vec<char>> = Vec::new();
    for col in 0..columns {
        let mut column_state: Vec<char> = vec![];
        for row in 0..rows {
            match inital_condition_raw[row][col] {
                Some(x) => column_state.push(x),
                None => ()
            }
        }
        column_state.reverse();
        inital_conditons.push(column_state);
    }
    // All the rest of the input is moves, parse them by spliting by spaces and using hardcoded
    // indexces
    let mut moves = vec![];
    for line in lines {
        let line = line.expect("Error reading moves");
        let line: Vec<_> = line.split(' ').collect();
        let items_moved = line[1].parse::<usize>().unwrap();
        let from = line[3].parse::<usize>().unwrap();
        let to = line[5].parse::<usize>().unwrap();
        moves.push((items_moved, from, to))
    }
    // Finaly, we can continue on to part 1
    {
        let mut conditions = inital_conditons.clone();
        for (count, from, to) in &moves {
            for _ in 0..*count {
                let poped = conditions[from - 1].pop().expect("Attempted to remove from an empty column");
                conditions[to - 1].push(poped);
            }
        }
        let mut answer = String::new();
        for mut col in conditions {
            answer.push(col.pop().expect("Column is empty after all moves"));
        }
        println!("part 1 {}", answer);
    }
    // For part 2, the craine move multiple blocks
    {
        let mut conditions = inital_conditons.clone();
        for (count, from, to) in &moves {
            let mut poped = vec![];
            for _ in 0..*count {
                poped.push(conditions[from - 1].pop().expect("Attempted to remove from an empty column"));
            }
            poped.reverse();
            for b in poped {
                conditions[to - 1].push(b);
            }
        }
        let mut answer = String::new();
        for mut col in conditions {
            answer.push(col.pop().expect("Column is empty after all moves"));
        }
        println!("part 2 {}", answer);
    }
}
