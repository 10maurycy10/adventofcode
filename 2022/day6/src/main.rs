use std::collections::HashSet;

fn solve(input: &[char], len: usize) -> Option<usize> {
    for startidx in 0..(input.len() - 3) {
        let slice = &input[startidx..(startidx+len)];
        let set = slice.iter().collect::<HashSet<_>>();
        if (set.len() == len) {
            return Some(startidx + len)
        }
    }
    return None
}

fn main() {
    let mut input = Vec::new();

    for line in std::io::stdin().lines() {
        let mut line = line.unwrap().chars().collect::<Vec<_>>();
        input.append(&mut line);
    }

    println!("Part one: {:?}", solve(&input[..], 4));
    // That's *it*, what!
    println!("Part two: {:?}", solve(&input[..], 14));
}
