#[cfg(test)]
mod tests {
    use indoc::indoc;
    use regex::Regex;
    use std::str::Lines;

    #[derive(Debug, Default)]
    struct Coord {
        x: u32,
        y: u32,
    }

    #[derive(Debug, Default)]
    struct ClawMachine {
        button_a_inc: Coord,
        button_b_inc: Coord,
        prize_location: Coord,
    }

    impl ClawMachine {
        const BUTTON_A_TOKEN_PRICE: u32 = 3;
        const BUTTON_B_TOKEN_PRICE: u32 = 1;

        fn calculate_cheapest_way_to_win_in_tokens(self) -> Option<u32> {
            let x_a = self.button_a_inc.x;
            let y_a = self.button_a_inc.y;

            let x_b = self.button_b_inc.x;
            let y_b = self.button_b_inc.y;

            let x_p = self.prize_location.x;
            let y_p = self.prize_location.y;

            let mut res = u32::MAX;

            // Naive solution
            for button_a_press_count in 1..=100 {
                let x = x_a * button_a_press_count;
                let y = y_a * button_a_press_count;

                for button_b_press_count in 1..=100 {
                    let x = x + x_b * button_b_press_count;
                    let y = y + y_b * button_b_press_count;

                    if x == x_p && y == y_p {
                        let new_res = button_a_press_count * Self::BUTTON_A_TOKEN_PRICE
                            + button_b_press_count * Self::BUTTON_B_TOKEN_PRICE;

                        res = res.min(new_res);
                    }
                }
            }

            if res != u32::MAX {
                Some(res)
            } else {
                None
            }
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

                Some(item)
            } else {
                None
            }
        }
    }

    pub fn solution(input: &str) -> u32 {
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

        assert_eq!(480, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(33921, solution(&input));
    }
}
