use std::collections::HashMap;

fn main() {
    let mut files: HashMap<String, usize> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    for command in std::io::stdin().lines() {
        let command = command.unwrap();
        match command.as_str() {
            // ls is never called with arguments, and is the only command that produces output. We
            // can just ignore it
            "$ ls" => (),
            // No filepaths include .. execpt just .., so we can trivial implement it
            "$ cd .." => {path.pop();}, 
            // Handle all other invocations of cd my adding hre filename onto the path
            command if command.starts_with("$ cd ") => {
                let name = &command[5..];
                if name.len() > 0 {
                    path.push(name.to_string());
                }
            }
            // Handle command output, containing filenames and sizes
            file => {
                let (prefix, name) = file.split_once(' ').unwrap();
                if prefix == "dir" {
                    // For directories, simple ensure that they exist in the hashmap
                    files.entry(path.join("/")).or_insert(0);
                } else {
                    // For files, parse the size and add it to the size of the file, and the size
                    // of all *directories* that it is a part of
                    let size = prefix.parse::<usize>().unwrap();
                    let mut filepath = path.clone();
                    filepath.push(name.to_string());
                    for i in 1..filepath.len() {
                        *files.entry(filepath[0..i].join("/")).or_default() += size;
                    }
                    
                }
            }
        }
    }

    let part1: usize = files.values().filter(|&&x| x<=100000).sum();
    println!("Part 1: {}", part1);


    let used = files.get("/").unwrap();
    let total = 70000000;
    let required = 30000000;
    let needed = required - (total - used);
    let mut sizes: Vec<usize> = files.values().copied().collect();
    sizes.sort();
    let part2 = sizes.iter().filter(|&&x| x>=needed).next().unwrap();
    println!("Part 2: {}", part2);
}
