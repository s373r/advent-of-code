#[cfg(test)]
mod tests {
    use indoc::indoc;
    use regex::Regex;
    use std::str::Lines;

    #[derive(Debug, Default)]
    struct Coord {
        x: u64,
        y: u64,
    }

    #[derive(Debug, Default)]
    struct ClawMachine {
        button_a_inc: Coord,
        button_b_inc: Coord,
        prize_location: Coord,
    }

    impl ClawMachine {
        const BUTTON_A_TOKEN_PRICE: u64 = 3;
        const BUTTON_B_TOKEN_PRICE: u64 = 1;

        fn calculate_cheapest_way_to_win_in_tokens(self) -> Option<u64> {
            // Cramer's rule
            // https://en.wikipedia.org/wiki/Cramer's_rule#Explicit_formulas_for_small_systems
            //
            // Button A: X+94, Y+34
            // Button B: X+22, Y+67
            // Prize: X=8400, Y=5400
            //
            // A = (94 22)    B = (8400)    X = (x1)
            //     (34 67)        (5400)        (x2)
            let a11 = self.button_a_inc.x as i64;
            let a12 = self.button_b_inc.x as i64;
            let a21 = self.button_a_inc.y as i64;
            let a22 = self.button_b_inc.y as i64;
            let b1 = self.prize_location.x as i64;
            let b2 = self.prize_location.y as i64;

            let d_a = a11 * a22 - a12 * a21;

            let x1 = ((b1 * a22 - a12 * b2) / d_a).unsigned_abs();
            let x2 = ((a11 * b2 - b1 * a21) / d_a).unsigned_abs();

            // Post-check
            let button_a_press_count = x1;
            let button_b_press_count = x2;

            let x = self.button_a_inc.x * button_a_press_count
                + self.button_b_inc.x * button_b_press_count;
            let y = self.button_a_inc.y * button_a_press_count
                + self.button_b_inc.y * button_b_press_count;

            if x != self.prize_location.x || y != self.prize_location.y {
                return None;
            }

            Some(
                button_a_press_count * Self::BUTTON_A_TOKEN_PRICE
                    + button_b_press_count * Self::BUTTON_B_TOKEN_PRICE,
            )
        }
    }

    struct ClawMachineIter<'a> {
        lines_it: Lines<'a>,
        coord_re: Regex,
    }

    impl<'a> ClawMachineIter<'a> {
        fn new(input: &'a str) -> Self {
            Self {
                lines_it: input.lines(),
                coord_re: Regex::new(r"X.(\d+), Y.(\d+)").unwrap(),
            }
        }
    }

    impl Iterator for ClawMachineIter<'_> {
        type Item = ClawMachine;

        fn next(&mut self) -> Option<Self::Item> {
            if let (Some(button_a_inc_str), Some(button_b_inc_str), Some(prize_location_str)) = (
                self.lines_it.next(),
                self.lines_it.next(),
                self.lines_it.next(),
            ) {
                // Skip the empty separator line, if any
                self.lines_it.next();

                let mut item = ClawMachine::default();

                for (_, [x, y]) in self
                    .coord_re
                    .captures_iter(button_a_inc_str)
                    .map(|c| c.extract())
                {
                    item.button_a_inc.x = x.parse().unwrap();
                    item.button_a_inc.y = y.parse().unwrap();
                }
                for (_, [x, y]) in self
                    .coord_re
                    .captures_iter(button_b_inc_str)
                    .map(|c| c.extract())
                {
                    item.button_b_inc.x = x.parse().unwrap();
                    item.button_b_inc.y = y.parse().unwrap();
                }
                for (_, [x, y]) in self
                    .coord_re
                    .captures_iter(prize_location_str)
                    .map(|c| c.extract())
                {
                    item.prize_location.x = x.parse().unwrap();
                    item.prize_location.y = y.parse().unwrap();
                }

                item.prize_location.x += 10000000000000;
                item.prize_location.y += 10000000000000;

                Some(item)
            } else {
                None
            }
        }
    }

    pub fn solution(input: &str) -> u64 {
        ClawMachineIter::new(input).fold(0, |acc, item| {
            acc + item.calculate_cheapest_way_to_win_in_tokens().unwrap_or(0)
        })
    }

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
            "#
        );

        assert_eq!(875318608908, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(82261957837868, solution(&input));
    }
}
