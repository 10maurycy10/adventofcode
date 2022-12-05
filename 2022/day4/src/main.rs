fn contains(a: &[u32], b: &[u32]) -> bool {
    a[0] <= b[0] && a[1] >= b[1]
}

fn overlaps(a: &[u32], b: &[u32]) -> bool {
    if (contains(a, b) || contains(b, a)) {
        return true
    }
    // Ensure ordering
    if a[0] > b[0] {
        return overlaps(b, a);
    } 
    return a[1] >= b[0] 
}

fn main() {
    let input: Vec<Vec<Vec<u32>>> = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|x| x.split('-').map(|y| y.parse::<u32>().unwrap()).collect::<Vec<u32>>())
                .collect()
        })
        .collect();

    let solution_to_part_1 = input.iter().filter(|a| contains(&a[0], &a[1]) || contains(&a[1], &a[0])).count();
    println!("{}", solution_to_part_1);
    let solution_to_part_2 = input.iter().filter(|a| overlaps(&a[0], &a[1])).count();
    println!("{}", solution_to_part_2);
}
