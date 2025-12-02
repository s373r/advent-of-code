// The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?
//
// Now, an ID is invalid if it is made only of some sequence of digits repeated *at least* twice. So, `12341234` (`1234` two times), `123123123` (`123` three times), `1212121212` (`12` five times), and `1111111` (`1` seven times) are all invalid IDs.
//
// From the same example as before:
//
// * `11-22` still has two invalid IDs, `*11*` and `*22*`.
// * `95-115` now has two invalid IDs, `*99*` and `*111*`.
// * `998-1012` now has two invalid IDs, `*999*` and `*1010*`.
// * `1188511880-1188511890` still has one invalid ID, `*1188511885*`.
// * `222220-222224` still has one invalid ID, `*222222*`.
// * `1698522-1698528` still contains no invalid IDs.
// * `446443-446449` still has one invalid ID, `*446446*`.
// * `38593856-38593862` still has one invalid ID, `*38593859*`.
// * `565653-565659` now has one invalid ID, `*565656*`.
// * `824824821-824824827` now has one invalid ID, `*824824824*`.
// * `2121212118-2121212124` now has one invalid ID, `*2121212121*`.
//
// Adding up all the invalid IDs in this example produces `*4174379265*`.
//
// *What do you get if you add up all of the invalid IDs using these new rules?*

fn is_invalid_id(n: u64) -> bool {
    let n_as_str = n.to_string();

    for i in 1..n_as_str.len() {
        let n_part_as_str = &n_as_str[0..i];

        if n_as_str.matches(n_part_as_str).count() * n_part_as_str.len() == n_as_str.len() {
            return true;
        }
    }

    false
}

fn invalid_ids_sum_from_range(start: u64, end: u64) -> u64 {
    (start..=end).fold(0, |mut acc, id| {
        if is_invalid_id(id) {
            acc += id;
        }
        acc
    })
}

fn invalid_ids_sum(input: &str) -> u64 {
    let ids_ranges = input
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            (start.parse().unwrap(), end.trim_end().parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    ids_ranges
        .iter()
        .map(|range| invalid_ids_sum_from_range(range.0, range.1))
        .sum()
}

#[test]
fn test_example() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    assert_eq!(4174379265, invalid_ids_sum(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(46769308485, invalid_ids_sum(&input));
}
