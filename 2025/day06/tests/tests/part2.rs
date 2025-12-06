// \--- Part Two ---
// ----------
//
// The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.
//
// Cephalopod math is written *right-to-left in columns*. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)
//
// Here's the example worksheet again:
//
// ```
// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +
//
// ```
//
// Reading the problems right-to-left one column at a time, the problems are now quite different:
//
// * The rightmost problem is `4` + `431` + `623` = `*1058*`
// * The second problem from the right is `175` \* `581` \* `32` = `*3253600*`
// * The third problem from the right is `8` + `248` + `369` = `*625*`
// * Finally, the leftmost problem is `356` \* `24` \* `1` = `*8544*`
//
// Now, the grand total is `1058` + `3253600` + `625` + `8544` = `*3263827*`.
//
// Solve the problems on the math worksheet again. *What is the grand total found by adding together all of the answers to the individual problems?*

fn solution(input: &str) -> u64 {
    let mut number_lines = Vec::new();
    let mut op_triplets = Vec::new();

    for line in input.lines() {
        let is_number_line = line.chars().any(|c| c.is_ascii_digit());
        if is_number_line {
            // Will be processed later when we know the indices
            number_lines.push(line)
        } else {
            let mut char_iter = line
                .char_indices()
                .filter(|(_, c)| !c.is_whitespace())
                .peekable();
            op_triplets = std::iter::from_fn(|| {
                let (start_digit_idx, op) = char_iter.next()?;
                let end_digit_idx = char_iter
                    .peek()
                    .map(|(i, _)| {
                        *i - 2 // -2: -1 for the previous char & -1 for padding between columns
                    })
                    .unwrap_or(line.len() - 1);
                Some((op, start_digit_idx, end_digit_idx))
            })
            .collect();
        }
    }

    use std::ops::{Add, Mul};

    op_triplets
        .into_iter()
        .map(|(op, start_digit_idx, end_digit_idx)| {
            let (init, op_fn): (u64, fn(u64, u64) -> u64) = match op {
                '+' => (0, u64::add),
                '*' => (1, u64::mul),
                _ => unreachable!(),
            };

            (start_digit_idx..=end_digit_idx)
                .rev()
                .fold(init, |acc, digit_idx| {
                    let number = (0..number_lines.len())
                        .map(|line_idx| {
                            number_lines[line_idx]
                                .chars()
                                .nth(digit_idx)
                                .and_then(|c| c.to_digit(10))
                        })
                        .fold(0, |acc, maybe_number_digit| {
                            if let Some(number_digit) = maybe_number_digit {
                                // Add a digit to left side of number
                                10 * acc + number_digit as u64
                            } else {
                                acc
                            }
                        });

                    op_fn(acc, number)
                })
        })
        .sum()
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +  "#
    );

    assert_eq!(3263827, solution(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(12608160008022, solution(&input));
}
