// --- Part Two ---
//
// The engineers are surprised by the low number of safe reports until they realize they forgot to tell you
// about the Problem Dampener.
//
// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems *tolerate a single bad level*
// in what would otherwise be a safe report. It's like the bad level never happened!
//
// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe,
// the report instead counts as safe.
//
// More of the above example's reports are now safe:
//
// * `7 6 4 2 1`: *Safe* without removing any level.
// * `1 2 7 8 9`: *Unsafe* regardless of which level is removed.
// * `9 7 6 2 1`: *Unsafe* regardless of which level is removed.
// * `1 *3* 2 4 5`: *Safe* by removing the second level, `3`.
// * `8 6 *4* 4 1`: *Safe* by removing the third level, `4`.
// * `1 3 6 7 9`: *Safe* without removing any level.
//
// Thanks to the Problem Dampener, `*4*` reports are actually *safe*!
//
// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports.
// *How many reports are now safe?*

pub fn safe_reports_count(input: &str) -> u64 {
    let mut res = 0;

    for line in input.lines() {
        let report = line
            .split(' ')
            .flat_map(|l| l.parse::<i32>())
            .collect::<Vec<_>>();

        if is_safe_report(report.iter()) {
            res += 1;
            continue;
        }

        for skip_index in 0..report.len() {
            let updated_report = report.iter().enumerate().filter_map(|(i, value)| {
                if i != skip_index {
                    Some(value)
                } else {
                    None
                }
            });

            if is_safe_report(updated_report) {
                res += 1;
                break;
            }
        }
    }

    res as u64
}

fn is_safe_report<'a>(report: impl Iterator<Item = &'a i32>) -> bool {
    let mut prev_increasing = None;
    let mut prev_level = 0;
    let mut first_level = true;

    for level in report {
        let delta = level - prev_level;

        if !first_level {
            let increasing = prev_level < *level;

            if !(1..=3).contains(&delta.abs()) {
                return false;
            }

            if let Some(prev_increasing) = prev_increasing {
                if prev_increasing && !increasing {
                    return false;
                }

                if !prev_increasing && increasing {
                    return false;
                }
            }

            prev_increasing = Some(increasing);
        }

        first_level = false;
        prev_level = *level;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "#
        );

        assert_eq!(4, safe_reports_count(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(290, safe_reports_count(&input));
    }
}
