#[cfg(test)]
mod tests {
    use indoc::indoc;

    pub fn solution(input: &str) -> u64 {
        #[derive(Debug, Copy, Clone)]
        enum Token {
            Number(u64),
            Operator(OperatorToken),
        }

        #[derive(Debug, Copy, Clone)]
        enum OperatorToken {
            Plus,
            Multiply,
            Concatenation,
        }

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
                let operators_count = numbers.len() - 1;
                let operator_combinations_count = 3u32.pow(operators_count as u32);

                for operator_combination in 0..operator_combinations_count {
                    let mut numbers_it = numbers.iter();

                    let mut tokens = Vec::with_capacity(numbers.len() + operators_count);

                    tokens.push(Token::Number(*numbers_it.next().unwrap()));

                    let mut operators_it = (0..operators_count).map(|i| {
                        let mut n = operator_combination;

                        let mut r = 0;
                        for _ in 0..i + 1 {
                            r = n % 3;
                            n /= 3;
                        }

                        match r {
                            0 => OperatorToken::Plus,
                            1 => OperatorToken::Multiply,
                            2 => OperatorToken::Concatenation,
                            _ => unreachable!(),
                        }
                    });

                    while let (Some(number), Some(operator)) =
                        (numbers_it.next(), operators_it.next())
                    {
                        tokens.push(Token::Operator(operator));
                        tokens.push(Token::Number(*number));
                    }

                    let mut tokens_it = tokens.into_iter();

                    let Token::Number(first_number) = tokens_it.next().unwrap() else {
                        unreachable!();
                    };
                    let mut actual_equation_res = first_number;

                    while let (Some(number_token), Some(operator_token)) =
                        (tokens_it.next(), tokens_it.next())
                    {
                        match (number_token, operator_token) {
                            (Token::Operator(operator), Token::Number(number)) => match operator {
                                OperatorToken::Plus => actual_equation_res += number,
                                OperatorToken::Multiply => actual_equation_res *= number,
                                OperatorToken::Concatenation => {
                                    let number_len = number.checked_ilog10().unwrap_or(0) + 1;

                                    actual_equation_res *= 10u64.pow(number_len);
                                    actual_equation_res += number;
                                }
                            },
                            _ => {
                                unreachable!()
                            }
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

        assert_eq!(11387, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(106016735664498, solution(&input));
    }
}
