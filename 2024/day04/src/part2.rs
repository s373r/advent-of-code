#[cfg(test)]
mod tests {
    use indoc::indoc;

    pub fn xmas_counter(input: &str) -> u64 {
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut res = 0;

        let x_m = data[0].len();
        let y_m = data.len();

        for y in 0..y_m {
            for x in 0..x_m {
                let c = data[y][x];

                if c == 'A' {
                    let Some(top) = y.checked_sub(1) else {
                        continue;
                    };

                    let bottom = y + 1;
                    if bottom == y_m {
                        continue;
                    }

                    let Some(left) = x.checked_sub(1) else {
                        continue;
                    };

                    let right = x + 1;
                    if right == x_m {
                        continue;
                    }

                    let c_tl = data[top][left];
                    let c_tr = data[top][right];
                    let c_bl = data[bottom][left];
                    let c_br = data[bottom][right];

                    match (c_tl, c_tr, c_br, c_bl) {
                        ('S', 'M', 'M', 'S') => res += 1,
                        ('S', 'S', 'M', 'M') => res += 1,
                        ('M', 'S', 'S', 'M') => res += 1,
                        ('M', 'M', 'S', 'S') => res += 1,
                        _ => {}
                    }
                }
            }
        }

        res
    }

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            .M.S......
            ..A..MSMS.
            .M.S.MAA..
            ..A.ASMSM.
            .M.S.M....
            ..........
            S.S.S.S.S.
            .A.A.A.A..
            M.M.M.M.M.
            ..........
            "#
        );

        assert_eq!(9, xmas_counter(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(1_885, xmas_counter(&input));
    }
}
