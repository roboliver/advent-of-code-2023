use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", calibration_sum(&input));
}

fn calibration_sum(input: &str) -> u32 {
    input.lines()
        .map(|line| calibration_value(line))
        .sum()
}

fn calibration_value(line: &str) -> u32 {
    let digits = line.chars()
        .filter(|char| char.is_digit(10))
        .map(|char| char.to_digit(10).expect("Not a digit"))
        .collect::<Vec<u32>>();
    10 * digits.first().expect("No first digit") + digits.last().expect("No last digit")
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";
        assert_eq!(142, calibration_sum(input));
    }
}