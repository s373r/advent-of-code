// \--- Day 4: Printing Department ---
// ----------
//
// You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).
//
// Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.
//
// "Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."
//
// If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.
//
// The rolls of paper (`@`) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.
//
// For example:
//
// ```
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
// ```
//
// The forklifts can only access a roll of paper if there are *fewer than four rolls of paper* in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.
//
// In this example, there are `*13*` rolls of paper that can be accessed by a forklift (marked with `x`):
//
// ```
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
// ```
//
// Consider your complete diagram of the paper roll locations. *How many rolls of paper can be accessed by a forklift?*

fn solution(input: &str, w: usize, h: usize) -> usize {
    let mut result = 0;

    let m = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

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
                result += 1;
            }
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

    assert_eq!(13, solution(input, 10, 10));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(1478, solution(&input, 140, 140));
}
