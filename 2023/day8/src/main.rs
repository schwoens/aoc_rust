use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part1: {}", part1(input));
    println!("Part 2: {}", part2(input));
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

fn part2(input: &str) -> usize {
    let (directions, nodes_str) = input.split_once("\n\n").unwrap();
    let mut nodes = HashMap::new();
    let mut current_nodes = Vec::new();
    for line in nodes_str.lines() {
        let (label, node) = line.split_once(" = ").unwrap();
        let node = node.trim_matches(['(', ')']);
        let (left, right) = node.split_once(", ").unwrap();

        if label.ends_with('A') {
            current_nodes.push(label);
        }

        nodes.insert(label, (left, right));
    }

    let mut arrival_times = Vec::new();
    'outer: for current_node in current_nodes {
        let mut current_node = current_node;
        let mut counter = 0;
        loop {
            for direction in directions.chars() {
                match direction {
                    'L' => current_node = nodes[current_node].0,
                    'R' => current_node = nodes[current_node].1,
                    _ => panic!("illegal direction"),
                }
                counter += 1;
                if current_node.ends_with('Z') {
                    arrival_times.push(counter);
                    continue 'outer;
                }
            }
        }
    }
    arrival_times.into_iter().reduce(lcm).unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part1(test_input), 2);
    }

    #[test]
    fn part2_works() {
        let test_input = include_str!("../test_input2.txt");
        assert_eq!(part2(test_input), 6);
    }
}
