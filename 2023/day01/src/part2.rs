// --- Part Two ---
//
// Your calculation isn't quite right. It looks like some of the digits are
// actually spelled out with letters: `one`, `two`, `three`, `four`, `five`, `six`, `seven`,
// `eight`, and `nine` also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and
// last digit on each line. For example:
//
// ```
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// ```
//
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
// Adding these together produces 281.
//
// What is the sum of all of the calibration values?

use std::io::{BufRead, BufReader, Read};

use regex::Regex;

fn parse_digit(value: &str) -> u32 {
    let result = match value {
        "one" | "eno" | "1" => 1,
        "two" | "owt" | "2" => 2,
        "three" | "eerht" | "3" => 3,
        "four" | "ruof" | "4" => 4,
        "five" | "evif" | "5" => 5,
        "six" | "xis" | "6" => 6,
        "seven" | "neves" | "7" => 7,
        "eight" | "thgie" | "8" => 8,
        "nine" | "enin" | "9" => 9,
        _ => panic!(),
    };

    result
}

fn extract_line_calibration_value(line: &str) -> u32 {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let re_reverse = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();
    let reversed_line: String = line.chars().rev().collect();
    let first_digit = re.find(line).map(|a| parse_digit(a.as_str())).unwrap();
    let last_digit = re_reverse
        .find(reversed_line.as_str())
        .map(|a| parse_digit(a.as_str()))
        .unwrap();

    first_digit * 10 + last_digit
}

pub fn calibration_value_sum<R: Read>(buffer: BufReader<R>) -> u32 {
    buffer
        .lines()
        // NOTE: Option<T> -> T
        .flatten()
        .map(|line| extract_line_calibration_value(&line))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn test_simple() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        let input_buffer = BufReader::new(input.as_bytes());

        assert_eq!(calibration_value_sum(input_buffer), 281);
    }
    #[test]
    fn test_merged_words() {
        let input = r#"2eight2kqmmbsbjvxtvjhponesixtwonesn"#;
        // NOTE:                                        ^^^^^
        //                                              twone
        let input_buffer = BufReader::new(input.as_bytes());

        assert_eq!(calibration_value_sum(input_buffer), 21);
    }

    #[test]
    fn test_real_input() {
        let input_buffer = read_input("input.txt").unwrap();

        assert_eq!(calibration_value_sum(input_buffer), 56_017);
    }
}
