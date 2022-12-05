use std::collections::HashSet;

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}

fn main() {
    // Input parser
    let mut sacks: Vec<(Vec<char>,Vec<char>)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line: Vec<char> = line.chars().collect();
        assert!(line.len() % 2 == 0);
        let pivot = line.len() / 2;
        let c1 = &line[0..pivot];
        let c2 = &line[pivot..line.len()];
        assert_eq!(c1.len(), c2.len());
        sacks.push((c1.to_vec(),c2.to_vec()));
    }
    // Find intersections
    let mut intersecting: Vec<Vec<char>> = sacks
        .iter()
        .map(|(c1, c2)| (c1.iter().collect::<HashSet<_>>(), c2.iter().collect::<HashSet<_>>()))
        .map(|(c1, c2)| c1.intersection(&c2).map(|&&x| x).collect())
        .collect();
    // Sum the prio of all intersecting items
    let solution_part_1: u32 = intersecting.iter().flatten().map(|c| priority(*c)).sum();
    println!("Part One solution: {:?}", solution_part_1);
    // Find all groups badges
    {
        let mut badges = vec![];
        let stacks = sacks.clone();
        let mut rem = &stacks[..];
        while rem.len() > 0 {
            let group = &rem[..3];
            rem = &rem[3..];
            let badge = group
                .iter()
                .map(|sack| sack.0.iter().chain(sack.1.iter()).collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).map(|&x| x).collect())
                .expect("Missing badges");
            badges.push(badge);
        }
        let solutions_part_2: u32 = badges.iter().flatten().map(|c| priority(**c)).sum();
        println!("Part Two solution: {:?}", solutions_part_2);
    }
}
