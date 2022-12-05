use std::io::BufRead;

fn main() {
    let mut total_counts = vec![];

    // We are only intested in the total amount of caloies on each elf, not the consituant sacks,
    // so parsing and summing is done together. This varable holds the count of the current elf
    // recored being read.
    let mut total: usize = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            // If the line is empty, add the total to the list and reset the count
            total_counts.push(total);
            total = 0;
        } else {
            // If it is not, add it to the count for the current elf.
            total += line.parse::<usize>().expect("Invalid number on non-empty line.")
        }
    }
    // Avoid missing the last elf, this could leed to phantom, 0 calorie, elf appering if there is a empty at
    // the end of the list.
    total_counts.push(total);

    // Sort the list in decending order to find the elf with the most calloies
    total_counts.sort();
    total_counts.reverse();

    // Find the top elf, and answer part 1
    println!("Answer for part one is: {}",total_counts[0]);

    // Find the top 3 elves, and answer part 2
    println!("Answer for part two is: {}", (&total_counts[0..3]).iter().sum::<usize>());
}
