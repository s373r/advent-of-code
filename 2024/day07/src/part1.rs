#[cfg(test)]
mod tests {
    use indoc::indoc;

    pub fn solution(input: &str) -> u64 {
        input
            .lines()
            .map(|line| {
                let mut split_t = line.split(": ");
                let equation_res = split_t.next().unwrap().parse::<u64>().unwrap();
                let numbers = split_t
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                (equation_res, numbers)
            })
            .fold(0, |acc, (expected_equation_res, numbers)| {
                let operator_combinations_count = 2u32.pow(numbers.len() as u32 - 1);

                for operator_combination in 0..operator_combinations_count {
                    let mut numbers_it = numbers.iter();

                    let mut actual_equation_res = *numbers_it.next().unwrap();

                    let mut operators_it = (0..operator_combinations_count).map(|i| {
                        // bit
                        (operator_combination >> i) & 1
                    });

                    while let (Some(number), Some(operator)) =
                        (numbers_it.next(), operators_it.next())
                    {
                        if operator == 0 {
                            actual_equation_res += number;
                        } else {
                            actual_equation_res *= number;
                        }
                    }

                    if expected_equation_res == actual_equation_res {
                        return acc + actual_equation_res;
                    }
                }

                acc
            })
    }

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "#
        );

        assert_eq!(3749, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(12940396350192, solution(&input));
    }
}
