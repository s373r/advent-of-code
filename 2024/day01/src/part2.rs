// --- Part Two ---
//
// Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.
//
// Or are they?
//
// The Historians can't agree on which group made the mistakes *or* how to read most of the Chief's handwriting,
// but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists!
// Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.
//
// This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
// Calculate a total *similarity score* by adding up each number in the left list after multiplying it by
// the number of times that number appears in the right list.
//
// Here are the same example lists again:
//
// ```
// 3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
//
// ```
//
// For these example lists, here is the process of finding the similarity score:
//
// * The first number in the left list is `3`. It appears in the right list three times,
//   so the similarity score increases by `3 * 3 = *9*`.
// * The second number in the left list is `4`. It appears in the right list once, so the similarity score
//   increases by `4 * 1 = *4*`.
// * The third number in the left list is `2`. It does not appear in the right list, so the similarity
//   score does not increase (`2 * 0 = 0`).
// * The fourth number, `1`, also does not appear in the right list.
// * The fifth number, `3`, appears in the right list three times; the similarity score increases by `*9*`.
// * The last number, `3`, appears in the right list three times; the similarity score again increases by `*9*`.
//
// So, for these example lists, the similarity score at the end of this process is `*31*` (`9 + 4 + 0 + 0 + 9 + 9`).
//
// Once again consider your left and right lists. *What is their similarity score?*

use std::collections::HashMap;

pub fn similarity_score(input: &str) -> u64 {
    let lines_count = input.lines().count();

    let mut left_map = HashMap::with_capacity(lines_count);
    let mut right_map = HashMap::with_capacity(lines_count);

    for line in input.lines() {
        let mut split_it = line.split("   ");

        let left = split_it.next().unwrap().parse::<i32>().unwrap();
        let right = split_it.next().unwrap().parse::<i32>().unwrap();

        *left_map.entry(left).or_insert(0) += 1;
        *right_map.entry(right).or_insert(0) += 1;
    }

    let mut res = 0;

    for (left_value, left_count) in left_map {
        let right_count = right_map.get(&left_value).unwrap_or(&0);

        res += (left_value * right_count) * left_count;
    }

    res as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#
        );

        assert_eq!(similarity_score(input), 31);
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(similarity_score(&input), 23_981_443);
    }
}
