use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let ranges = input.trim().split(",");
    let ids = ranges.flat_map(explode_range);

    let mut invalid_sum = 0;
    for id in ids {
        if is_invalid(id) {
            invalid_sum += id;
        }
    }
    invalid_sum
}

fn explode_range(range: &str) -> Vec<usize> {
    let range_tuple = range.split_once("-").unwrap();
    let beginning = range_tuple.0.parse::<usize>().unwrap();
    let end = range_tuple.1.parse::<usize>().unwrap();

    let mut range = Vec::new();

    for id in beginning..=end {
       range.push(id); 
    }
    range
}

fn is_invalid(id: usize) -> bool {
    let id = id.to_string();

    let first_half = &id[0..id.len()/2];
    let second_half = &id[id.len()/2..];

    first_half == second_half
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_works() {
        let test_input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part1(&test_input), 1227775554);
    }
}
