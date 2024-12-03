#[cfg(test)]
mod tests {
    use regex::Regex;

    fn multiplications(input: &str) -> u64 {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
        //                             |     ^^^^^ ^^^^^   ^^^^^^^^ ^^^^^^^^^^^
        //                     Group № |         1     2          3           4
        let mut res = 0;
        let mut activated = true;

        for captures in re.captures_iter(input) {
            let maybe_left = captures.get(1).map(|m| m.as_str().parse::<u64>().unwrap());
            let maybe_right = captures.get(2).map(|m| m.as_str().parse::<u64>().unwrap());
            let maybe_do = captures.get(3).map(|_| true).unwrap_or(false);
            let maybe_do_not = captures.get(4).map(|_| true).unwrap_or(false);

            match (maybe_left, maybe_right, maybe_do, maybe_do_not) {
                (Some(left), Some(right), _, _) if activated => {
                    res += left * right;
                }
                (_, _, true, _) => activated = true,
                (_, _, _, true) => activated = false,
                _ => {}
            }
        }

        res
    }

    #[test]
    fn test_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, multiplications(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(76_911_921, multiplications(&input));
    }
}
