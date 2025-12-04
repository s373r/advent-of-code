// \--- Part Two ---
// ----------
//
// Now, the Elves just need help accessing as much of the paper as they can.
//
// Once a roll of paper can be accessed by a forklift, it can be *removed*. Once a roll of paper is removed, the forklifts might be able to access *more* rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?
//
// Starting with the same example as above, here is one way you could remove as many rolls of paper as possible, using highlighted `*@*` to indicate that a roll of paper is about to be removed, and using `x` to indicate that a roll of paper was just removed:
//
// ```
// Initial state:
// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
//
// Remove 13 rolls of paper:
// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
//
// Remove 12 rolls of paper:
// .......x..
// .@@.x.x.@x
// x@@@@...@@
// x.@@@@..x.
// .@.@@@@.x.
// .x@@@@@@.x
// .x.@.@.@@@
// ..@@@.@@@@
// .x@@@@@@@.
// ....@@@...
//
// Remove 7 rolls of paper:
// ..........
// .x@.....x.
// .@@@@...xx
// ..@@@@....
// .x.@@@@...
// ..@@@@@@..
// ...@.@.@@x
// ..@@@.@@@@
// ..x@@@@@@.
// ....@@@...
//
// Remove 5 rolls of paper:
// ..........
// ..x.......
// .x@@@.....
// ..@@@@....
// ...@@@@...
// ..x@@@@@..
// ...@.@.@@.
// ..x@@.@@@x
// ...@@@@@@.
// ....@@@...
//
// Remove 2 rolls of paper:
// ..........
// ..........
// ..x@@.....
// ..@@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@x.
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ...@@.....
// ..x@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ...x@.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ....x.....
// ...@@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// Remove 1 roll of paper:
// ..........
// ..........
// ..........
// ...x@@....
// ...@@@@...
// ...@@@@@..
// ...@.@.@@.
// ...@@.@@@.
// ...@@@@@..
// ....@@@...
//
// ```
//
// Stop once no more rolls of paper are accessible by a forklift. In this example, a total of `*43*` rolls of paper can be removed.
//
// Start with your original diagram. *How many rolls of paper in total can be removed by the Elves and their forklifts?*

fn solution(input: &str, w: usize, h: usize) -> usize {
    let mut result = 0;

    let mut m = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    loop {
        let mut to_remove = Vec::new();

        for j in 0..h {
            for i in 0..w {
                let char = m[j][i];

                if char != '@' {
                    continue;
                }

                let left = i.saturating_sub(1);
                let right = (w - 1).min(i + 1);
                let top = j.saturating_sub(1);
                let bottom = (h - 1).min(j + 1);

                let mut neighbors: u8 = 0;

                for di in left..=right {
                    for dj in top..=bottom {
                        if di == i && dj == j {
                            continue;
                        }

                        if m[dj][di] == '@' {
                            neighbors += 1;
                        }
                    }
                }

                if neighbors < 4 {
                    to_remove.push((j, i));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        result += to_remove.len();

        for (j, i) in to_remove {
            m[j][i] = '.';
        }
    }

    result
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        "#
    );

    assert_eq!(43, solution(input, 10, 10));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(9120, solution(&input, 140, 140));
}
