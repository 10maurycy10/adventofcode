use std::io::BufRead;

// 1 for rock
// 2 for paper
// 3 for sisors
fn parse_move(c: u8) -> u8 {
    match c {
        b'A' => 1,
        b'B' => 2,
        b'C' => 3,
        b'X' => 1,
        b'Y' => 2,
        b'Z' => 3,
        _ => unimplemented!(),
    }
}

fn main() {
    let mut moves:Vec<(u8, u8)> = vec![];

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap().into_bytes();
        let (oponent, you) = (parse_move(line[0]), parse_move(line[2]));
        println!("{}", you);
        moves.push((oponent, you));
    }
    
    let mut total_score = 0;
    for (op, you) in &moves {
        let outcome = match (op, you) {
            (1, 1) => 3, //Tie
            (2, 2) => 3, //Tie
            (3, 3) => 3, //Tie

            (1, 2) => 6, //we win
            (2, 3) => 6, //we win
            (3, 1) => 6, //we win
            
            (2, 1) => 0, //they win
            (3, 2) => 0, //they win
            (1, 3) => 0, //they win
            _ => unreachable!()
        };
        total_score += you + outcome;
    }
    println!("Total for part one is {}", total_score);

    let mut total_score = 0;
    for (op, outcome) in &moves {
        let ourmove = match (outcome, op) {
            // We lose
            (1, 1) => 3,
            (1, 2) => 1,
            (1, 3) => 2,
            // We tie
            (2, 1) => 1,
            (2, 2) => 2,
            (2, 3) => 3,
            // We win
            (3, 3) => 1,
            (3, 1) => 2,
            (3, 2) => 3,
            _ => unreachable!()
        };
        let outcome_score = (outcome - 1) * 3;
        total_score += outcome_score + ourmove;
    }
    println!("Total for part two is {}", total_score);
}
