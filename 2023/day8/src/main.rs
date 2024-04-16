use core::panic;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let (directions, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes = HashMap::new();
    for line in nodes_str.lines() {
        let (label, node) = line.split_once(" = ").unwrap();
        let node = node.trim_matches(['(', ')']);
        let (left, right) = node.split_once(", ").unwrap();

        nodes.insert(label, (left, right));
    }

    let mut current_node = "AAA";
    let mut counter = 0;

    loop {
        for direction in directions.chars() {
            match direction {
                'L' => current_node = nodes[current_node].0,
                'R' => current_node = nodes[current_node].1,
                _ => panic!("invalid direction"),
            }
            counter += 1;
            if current_node == "ZZZ" {
                return counter;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part1(test_input), 2);
    }
}
