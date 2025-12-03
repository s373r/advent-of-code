// \--- Part Two ---
// ----------
//
// The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the [static friction](https://en.wikipedia.org/wiki/Static_friction) of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.
//
// Now, you need to make the largest joltage by turning on *exactly twelve* batteries within each bank.
//
// The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be `*12*` digits in each bank's joltage output instead of two.
//
// Consider again the example from before:
//
// ```
// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
//
// ```
//
// Now, the joltages are much larger:
//
// * In `*987654321111*111`, the largest joltage can be found by turning on everything except some `1`s at the end to produce `*987654321111*`.
// * In the digit sequence `*81111111111*111*9*`, the largest joltage can be found by turning on everything except some `1`s, producing `*811111111119*`.
// * In `23*4*2*34234234278*`, the largest joltage can be found by turning on everything except a `2` battery, a `3` battery, and another `2` battery near the start to produce `*434234234278*`.
// * In `*8*1*8*1*8*1*911112111*`, the joltage `*888911112111*` is produced by turning on everything except some `1`s near the front.
//
// The total output joltage is now much larger: `987654321111` + `811111111119` + `434234234278` + `888911112111` = `*3121910778619*`.
//
// *What is the new total output joltage?*

fn largest_joltage_possible(battery: &str) -> u64 {
    const MAX_DIGIT_COUNT: usize = 12;

    let mut res = [0; MAX_DIGIT_COUNT];

    let digits = battery
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len = digits.len();

    let mut next_y = 0;

    for i in 0..MAX_DIGIT_COUNT {
        for y in next_y..=len - MAX_DIGIT_COUNT + i {
            let digit = digits[y];

            if res[i] < digit {
                res[i] = digit;
                next_y = y + 1;
            }
        }
    }

    // Digits array to number
    res.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn total_output_joltage(input: &str) -> u64 {
    input.lines().map(largest_joltage_possible).sum()
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
        "#
    );

    assert_eq!(3121910778619, total_output_joltage(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(167384358365132, total_output_joltage(&input));
}
