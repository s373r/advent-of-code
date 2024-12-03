#[cfg(test)]
mod tests {
    use regex::Regex;

    pub fn multiplications(input: &str) -> u64 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut res = 0;

        for (_, [left_str_arg, right_str_arg]) in re.captures_iter(input).map(|c| c.extract()) {
            let left = left_str_arg.parse::<u64>().unwrap();
            let right = right_str_arg.parse::<u64>().unwrap();

            res += left * right;
        }

        res
    }

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, multiplications(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(163_931_492, multiplications(&input));
    }
}
