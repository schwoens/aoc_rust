use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Part  1: {}", part1(&input));
}

fn part1(input: &str) -> i32 {
    let reports = parse_reports(input);
    count_safe_reports(&reports)
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

fn count_safe_reports(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .fold(0, |acc, report| if is_safe(report) { acc + 1 } else { acc })
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
}
