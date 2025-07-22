use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Part  1: {}", part1(&input));
    println!("Part  2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let reports = parse_reports(input);
    count_safe_reports(&reports, false)
}

fn part2(input: &str) -> i32 {
    let reports = parse_reports(input);
    count_safe_reports(&reports, true)
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|report| {
            report
                .split(" ")
                .map(|level| level.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn count_safe_reports(reports: &[Vec<i32>], dampener: bool) -> i32 {
    reports.iter().fold(0, |acc, report| {
        if is_safe(report) {
            acc + 1
        } else {
            if dampener {
                for i in 0..report.len() {
                    let mut dampened_report = report.clone();
                    dampened_report.remove(i);
                    if is_safe(&dampened_report) {
                        return acc + 1;
                    }
                }
            }
            acc
        }
    })
}

fn is_safe(report: &[i32]) -> bool {
    let mut report = report.iter();

    let mut altitude: i32 = 0;
    let mut current_altitude: i32;
    let mut current_level = report.next().unwrap();
    for next_level in report {
        current_altitude = next_level - current_level;
        if altitude == 0 {
            altitude = current_altitude;
        }

        if current_altitude == 0
            || (current_altitude > 0 && altitude < 0)
            || (current_altitude < 0 && altitude > 0)
            || current_altitude.abs() > 3
        {
            return false;
        }

        current_level = next_level;
    }
    true
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn part1_works() {
        let test_input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part1(&test_input), 2);
    }

    #[test]
    fn part2_works() {
        let test_input = read_to_string("test_input.txt").unwrap();
        assert_eq!(part2(&test_input), 4);
    }
}
