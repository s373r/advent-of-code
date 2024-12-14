#[cfg(test)]
mod tests {

    fn solution(input: &str, width: usize, height: usize, seconds: usize) -> usize {
        use regex::Regex;

        let mut map = vec![vec![0; width]; height];
        let re = Regex::new(r"-?\d+").unwrap();

        let robots_it = input.lines().map(|line| {
            let mut matches_it = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap());

            let x = matches_it.next().unwrap();
            let y = matches_it.next().unwrap();

            let dx = matches_it.next().unwrap();
            let dy = matches_it.next().unwrap();

            ((x, y), (dx, dy))
        });

        for ((x, y), (dx, dy)) in robots_it {
            let new_x = (x + dx * seconds as i32).rem_euclid(width as i32) as usize;
            let new_y = (y + dy * seconds as i32).rem_euclid(height as i32) as usize;

            map[new_y][new_x] += 1;
        }

        let middle_x = width / 2;
        let middle_y = height / 2;

        let mut top_left_corner = 0;
        let mut top_right_corner = 0;
        let mut bottom_left_corner = 0;
        let mut bottom_right_corner = 0;

        for (y, row) in map.into_iter().enumerate() {
            if y == middle_y {
                continue;
            }

            for (x, value) in row.into_iter().enumerate() {
                if x == middle_x {
                    continue;
                }

                if value == 0 {
                    continue;
                }

                let counter = match (y < middle_y, x < middle_x) {
                    (false, false) => &mut top_left_corner,
                    (false, true) => &mut top_right_corner,
                    (true, false) => &mut bottom_left_corner,
                    (true, true) => &mut bottom_right_corner,
                };

                *counter += value;
            }
        }

        let res = top_left_corner * top_right_corner * bottom_left_corner * bottom_right_corner;

        res as usize
    }

    #[test]
    fn test_example() {
        use indoc::indoc;

        let input = indoc!(
            r#"
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
            "#
        );

        assert_eq!(12, solution(input, 11, 7, 100));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(225810288, solution(&input, 101, 103, 100));
    }
}
