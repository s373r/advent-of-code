#[cfg(test)]
mod tests {
    use indoc::indoc;

    enum Direction {
        UP,
        Right,
        Down,
        Left,
    }

    pub fn solution(input: &str) -> u64 {
        let lines = input.lines().collect::<Vec<&str>>();
        let (mut x_0, mut y_0) = (0, 0);
        let mut data = Vec::with_capacity(lines.len());

        for (y, line) in lines.iter().enumerate() {
            let chars = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '^' {
                        x_0 = x;
                        y_0 = y;
                    }
                    c
                })
                .collect::<Vec<_>>();

            data.push(chars);
        }

        let x_m = data[0].len();
        let y_m = data.len();

        let mut x = x_0;
        let mut y = y_0;

        let mut d = Direction::UP;

        // Including the guard's starting position, ... (c)
        let mut res = 1;

        loop {
            let (next_x, next_y) = match d {
                Direction::UP => {
                    let Some(new_y) = y.checked_sub(1) else { break };
                    (x, new_y)
                }
                Direction::Right => {
                    let new_x = x + 1;
                    if new_x >= x_m {
                        break;
                    }
                    (new_x, y)
                }
                Direction::Down => {
                    let new_y = y + 1;
                    if new_y >= y_m {
                        break;
                    }
                    (x, new_y)
                }
                Direction::Left => {
                    let Some(new_x) = x.checked_sub(1) else { break };
                    (new_x, y)
                }
            };

            let next_c = data[next_y][next_x];

            match next_c {
                '#' => {
                    d = match d {
                        Direction::UP => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::UP,
                    };
                    continue;
                }
                'X' | '^' => { /* We've been here before */ }
                _ => {
                    data[next_y][next_x] = 'X';
                    res += 1
                }
            }

            x = next_x;
            y = next_y;
        }

        res
    }

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "#
        );

        assert_eq!(41, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(4_826, solution(&input));
    }
}
