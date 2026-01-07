use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("unable to read file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let lines = input.trim().split("\n");
    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for line in lines {

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i32>().unwrap();

        match direction {
            'R' => dial += distance,
            'L' => dial -= distance,
            _ => panic!("invalid instruction"),
        }

        if dial % 100 == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn part2(input: &str) -> usize {
    let lines = input.trim().split("\n");
    let mut dial: i32 = 50;
    let mut zero_count = 0;

    for line in lines {

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                'R' => dial += 1,
                'L' => dial -= 1,
                _ => panic!("invalid instruction"),
            }

            match dial {
                100 => dial = 0,
                -1 => dial = 99,
                _ => (),
            }

            if dial == 0 {
                zero_count += 1;
            }
        }
    }
    zero_count
}

#[cfg(test)]
pub mod test {
    use super::*;
    
    #[test]
    fn part1_works() {
        let test_input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part1(&test_input), 3);
    }

    #[test]
    fn part2_works() {
        let test_input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part2(&test_input), 6);
    }
}

