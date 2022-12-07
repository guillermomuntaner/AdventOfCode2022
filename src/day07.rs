use itertools::Itertools;
use std::collections::HashMap;

fn get_directory_sizes(input: &str) -> HashMap<String, usize> {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    // A ~3x perf optimization implemented here is to store the accumulated size in the stack to
    // avoid doing a lot of increments on the hash map. This looks a bit uglier, specially because
    // we need some code to consume leftovers after finishing looping over the input.
    let mut dirs_stack: Vec<(&str, usize)> = Vec::new();
    for line in input.lines() {
        if line.starts_with('$') {
            if line[2..].starts_with("cd") {
                let argument = &line[5..];
                if argument.eq("..") {
                    // Pop the dir from the stack, and store the data in the map.
                    let path = dirs_stack.iter().map(|(dir, _)| dir).join("");
                    let (_, size) = dirs_stack.pop().unwrap();
                    if size != 0 {
                        *dir_sizes.entry(path).or_insert(0) += size;
                    }
                } else {
                    dirs_stack.push((argument, 0));
                }
            }
        } else if !line[..3].eq("dir") {
            let size = line
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            for (_, dir_size) in &mut dirs_stack {
                *dir_size += size;
            }
        }
    }

    // Consume any left data in the dirs stack
    let mut path = String::new();
    for (dir, size) in dirs_stack.iter() {
        path.push_str(dir);
        *dir_sizes.entry(path.clone()).or_insert(0) += size;
    }

    dir_sizes
}

pub fn part1(input: &str) -> Option<usize> {
    let dir_sizes = get_directory_sizes(input);
    dir_sizes
        .values()
        .filter(|size| **size <= 100_000)
        .sum::<usize>()
        .into()
}

pub fn part2(input: &str) -> Option<usize> {
    let dir_sizes = get_directory_sizes(input);
    let used_space = dir_sizes["/"];
    let current_unused_space = 70000000 - used_space;
    let delete_target_size = 30000000 - current_unused_space;
    (*dir_sizes
        .values()
        .sorted()
        .find(|size| **size >= delete_target_size)
        .unwrap())
    .into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    const DAY: usize = 7;

    #[test]
    fn test_part1_example1() {
        assert_eq!(super::part1(&utils::read_example(DAY, 1)), Some(95437));
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(&utils::read_input(DAY)), Some(1513699));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(super::part2(&utils::read_example(DAY, 1)), Some(24933642));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(&utils::read_input(DAY)), Some(7991939));
    }
}
