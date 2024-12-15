#[cfg(test)]
mod tests {
    const PRINT_MOVEMENTS: bool = false;

    fn solution(input: &str) -> usize {
        fn inc(x: usize, dx: isize) -> usize {
            (x as isize + dx) as usize
        }

        let (warehouse_lines, movement_lines) = input
            .lines()
            .filter(|line| !line.is_empty())
            .partition::<Vec<_>, _>(|line| line.starts_with("#"));

        let mut x_0 = 0;
        let mut y_0 = 0;

        let mut warehouse = warehouse_lines
            .into_iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            x_0 = x;
                            y_0 = y;
                        }
                        c
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // for line in warehouse {
        //     println!("{line:?}");
        // }

        let movements_it = movement_lines.into_iter().flat_map(|line| line.chars());

        'movement: for movement in movements_it {
            if PRINT_MOVEMENTS {
                println!("-- movement: {movement} ---");
                for line in &warehouse {
                    println!("{line:?}");
                }
            }

            let (dx, dy) = match movement {
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                _ => unreachable!(),
            };

            let next_x = inc(x_0, dx);
            let next_y = inc(y_0, dy);

            match warehouse[next_y][next_x] {
                '#' => {
                    // Robot hits the wall.
                    continue;
                }
                '.' => {
                    warehouse[y_0][x_0] = '.';
                    warehouse[next_y][next_x] = '@';

                    x_0 = next_x;
                    y_0 = next_y;
                    continue;
                }
                'O' => {
                    // Attempting to move box(es)
                    let mut next_o_x = next_x;
                    let mut next_o_y = next_y;

                    loop {
                        match warehouse[next_o_y][next_o_x] {
                            'O' => {
                                // Behind the box is the next box. Let's keep looking.
                                next_o_x = inc(next_o_x, dx);
                                next_o_y = inc(next_o_y, dy);
                            }
                            '#' => {
                                // Nowhere to move
                                continue 'movement;
                            }
                            '.' => {
                                // Move box(es) over here.
                                break;
                            }
                            _ => unreachable!(),
                        };
                    }

                    warehouse[y_0][x_0] = '.';
                    warehouse[next_y][next_x] = '@';
                    warehouse[next_o_y][next_o_x] = 'O';

                    x_0 = next_x;
                    y_0 = next_y;
                }
                _ => unreachable!(),
            };
        }

        if PRINT_MOVEMENTS {
            println!("-- final ---");
            for line in &warehouse {
                println!("{line:?}");
            }
        }

        let mut res = 0;
        for (y, line) in warehouse.into_iter().enumerate() {
            for (x, c) in line.into_iter().enumerate() {
                if c != 'O' {
                    continue;
                }

                res += 100 * y + x;
            }
        }
        res
    }

    #[test]
    fn test_example_1() {
        use indoc::indoc;

        let input = indoc!(
            r#"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########
            
            <^^>>>vv<v>>v<<
            "#
        );

        assert_eq!(2028, solution(input));
    }

    #[test]
    fn test_example_2() {
        use indoc::indoc;

        let input = indoc!(
            r#"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########
            
            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
            "#
        );

        assert_eq!(10092, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(1552879, solution(&input));
    }
}
