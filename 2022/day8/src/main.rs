const DIRECTIONS: [(isize,isize); 4] = [(0,1), (0,-1), (1,0), (-1,0)];

fn is_vis(input: &[Vec<char>], pos: (usize, usize)) -> bool {
    let h = input[pos.0][pos.1];
    let mut vis_from_any = false;

    // For all directions
    for (dx, dy) in DIRECTIONS {
        // Check visivilty in that direction
        let mut vis = true;
        let mut x = (pos.0 as isize + dx) as usize;
        let mut y = (pos.1 as isize + dy) as usize;
        // Move allong the board until the end of the input
        // Checks for < 0 are ommited due to overflow
        while x < input.len() && y < input[x].len() {
            if input[x][y] < h {
                vis &= true;
            } else {
                vis &= false;
            }
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }
        vis_from_any |= vis;
    }
    return vis_from_any;
}

fn get_senic(input: &[Vec<char>], pos: (usize, usize)) -> usize {
    let mut overall_score = 1;
    let h = input[pos.0][pos.1];

    // For all directions
    for (dx, dy) in DIRECTIONS {
        let mut vis_in_dist = 0;
        let mut x = (pos.0 as isize + dx) as usize;
        let mut y = (pos.1 as isize + dy) as usize;
        // Checks for < 0 are ommited due to overflow
        while x < input.len() && y < input[x].len() {
            if input[x][y] >= h {
                vis_in_dist += 1;
                break;
            } else {
                vis_in_dist += 1;
            }
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }
        overall_score *= vis_in_dist;

    }

    return overall_score;
}

fn main() {
    let input: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();
    let mut total_vis: usize = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let vis = is_vis(&input, (x,y));
            if vis {
                total_vis+=1;
            }
        }
    }
    println!("Part one {}", total_vis);
    
    let mut best_score: usize = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            let score = get_senic(&input, (x,y));
            if score > best_score {
                best_score = score
            }
        }
    }
    println!("Part two {}", best_score);
}
