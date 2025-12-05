// \--- Part Two ---
// ----------
//
// The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.
//
// So that they can stop bugging you when they get new inventory, the Elves would like to know *all* of the IDs that the *fresh ingredient ID ranges* consider to be *fresh*. An ingredient ID is still considered fresh if it is in any range.
//
// Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:
//
// ```
// 3-5
// 10-14
// 16-20
// 12-18
//
// ```
//
// The ingredient IDs that these ranges consider to be fresh are `3`, `4`, `5`, `10`, `11`, `12`, `13`, `14`, `15`, `16`, `17`, `18`, `19`, and `20`. So, in this example, the fresh ingredient ID ranges consider a total of *`14`* ingredient IDs to be fresh.
//
// Process the database file again. *How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?*

use std::ops::RangeInclusive;

fn range_len(range: &RangeInclusive<u64>) -> u64 {
    range.end() - range.start() + 1
}

fn solution(input: &str) -> u64 {
    let mut ranges = Vec::<RangeInclusive<u64>>::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }

        let (start, end) = line.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        ranges.push(start..=end);
    }

    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));

    let mut result = 0;
    let mut ranges_iter = ranges.into_iter().peekable();

    while let Some(current) = ranges_iter.next() {
        let Some(next) = ranges_iter.peek_mut() else {
            // last range
            result += range_len(&current);
            break;
        };

        if current.start() <= next.end() && next.start() <= current.end() {
            // merge current into next
            *next = *current.start().min(next.start())..=*next.end().max(current.end());
        } else {
            // current not overlaps next
            result += range_len(&current);
        }
    }

    result
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
        "#
    );

    assert_eq!(14, solution(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(357608232770687, solution(&input));
}
