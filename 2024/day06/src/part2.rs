#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[derive(Clone, Copy)]
    enum Direction {
        UP,
        Right,
        Down,
        Left,
    }

    fn has_guard_stuck_in_loop(mut data: Vec<Vec<char>>, x_0: usize, y_0: usize) -> bool {
        let x_m = data[0].len();
        let y_m = data.len();

        let mut x = x_0;
        let mut y = y_0;

        let mut d = Direction::UP;

        loop {
            let (next_x, next_y) = match d {
                Direction::UP => {
                    let Some(new_y) = y.checked_sub(1) else {
                        return false;
                    };
                    (x, new_y)
                }
                Direction::Right => {
                    let new_x = x + 1;
                    if new_x >= x_m {
                        return false;
                    }
                    (new_x, y)
                }
                Direction::Down => {
                    let new_y = y + 1;
                    if new_y >= y_m {
                        return false;
                    }
                    (x, new_y)
                }
                Direction::Left => {
                    let Some(new_x) = x.checked_sub(1) else {
                        return false;
                    };
                    (new_x, y)
                }
            };

            let next_c = data[next_y][next_x];

            match (next_c, d) {
                ('#', _) => {
                    d = match d {
                        Direction::UP => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::UP,
                    };
                    continue;
                }
                ('U', Direction::UP) => return true,
                ('R', Direction::Right) => return true,
                ('D', Direction::Down) => return true,
                ('L', Direction::Left) => return true,
                _ => {
                    data[next_y][next_x] = match d {
                        Direction::UP => 'U',
                        Direction::Right => 'R',
                        Direction::Down => 'D',
                        Direction::Left => 'L',
                    };
                }
            }

            x = next_x;
            y = next_y;
        }
    }

    fn solution(input: &str) -> u64 {
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

        let mut res = 0;

        for (y, line) in data.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                // The new obstruction can't be placed at the guard's starting position ... (с)
                if x == x_0 && y == y_0 {
                    continue;
                }

                // Skip the attempt if there's already a wall there
                if data[y][x] == '#' {
                    continue;
                }

                let mut data_clone = data.clone();

                // Add an obstruction
                data_clone[y][x] = '#';

                if has_guard_stuck_in_loop(data_clone, x_0, y_0) {
                    res += 1;
                }
            }
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

        assert_eq!(6, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(1_721, solution(&input));
    }
}
