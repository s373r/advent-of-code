#[cfg(test)]
mod tests {
    use indoc::indoc;
    use regex::Regex;

    pub fn xmas_counter(input: &str) -> u64 {
        // Terrible hastily made solution, since it's nighttime
        let data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut res = 0;

        let mut lines = Vec::new();

        // Horizontal -
        for horizontal_line in &data {
            let s = horizontal_line.iter().collect::<String>();
            lines.push(s);
        }

        // Vertical |
        for x in 0..data[0].len() {
            let mut s = String::new();
            for vertical_line in &data {
                let c = vertical_line[x];
                s.push(c);
            }
            lines.push(s);
        }

        // Diagonal \
        let mut start_x = 0;
        let mut start_y = data.len() - 1;

        loop {
            let mut x = start_x;
            let mut y = start_y;

            let mut s = String::new();
            while x < data[0].len() && y < data.len() {
                let c = data[y][x];
                s.push(c);

                x += 1;
                y += 1;
            }

            lines.push(s);

            if start_y != 0 {
                start_y -= 1;
            } else if start_x != data[0].len() - 1 {
                start_x += 1
            } else {
                break;
            }
        }

        // Diagonal /
        let mut start_x = (data[0].len() - 1) as isize;
        start_y = data.len() - 1;

        loop {
            let mut x = start_x;
            let mut y = start_y;

            let mut s = String::new();
            while x >= 0 && y < data.len() {
                let c = data[y][x as usize];
                s.push(c);

                x -= 1;
                y += 1;
            }

            lines.push(s);

            if start_y != 0 {
                start_y -= 1;
            } else if start_x > 0 {
                start_x -= 1
            } else {
                break;
            }
        }

        for line in lines {
            for re in [
                Regex::new(r"(XMAS)").unwrap(),
                Regex::new(r"(SAMX)").unwrap(),
            ] {
                for _ in re.captures_iter(&line) {
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
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            "#
        );

        assert_eq!(18, xmas_counter(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(2_462, xmas_counter(&input));
    }
}
