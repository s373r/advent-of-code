#[cfg(test)]
mod tests {
    use indoc::indoc;
    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};

    pub fn solution(input: &str) -> u64 {
        let (ruleset_map, update_lines) = {
            let mut ruleset_map = HashMap::<_, HashSet<_>>::new();
            let mut update_lines = Vec::new();
            let mut process_rules = true;

            for line in input.lines() {
                if line.is_empty() {
                    process_rules = false;
                    continue;
                }

                if process_rules {
                    let mut rule_it = line.split('|');
                    let (Some(raw_left), Some(raw_right)) = (rule_it.next(), rule_it.next()) else {
                        unreachable!()
                    };
                    let left = raw_left.parse::<u64>().unwrap();
                    let right = raw_right.parse::<u64>().unwrap();

                    ruleset_map
                        .entry(left)
                        .and_modify(|value| {
                            value.insert(right);
                        })
                        .or_insert_with(|| HashSet::from([right]));
                } else {
                    let update = line
                        .split(',')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();

                    update_lines.push(update);
                }
            }
            (ruleset_map, update_lines)
        };

        let mut res = 0;

        for update_line_original in update_lines {
            let mut update_line = update_line_original.clone();

            update_line.sort_by(|left, right| {
                if let Some(right_rules) = ruleset_map.get(left) {
                    if right_rules.contains(right) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Greater
                }
            });

            if update_line == update_line_original {
                let middle_idx = (update_line.len() - 1) / 2;

                res += update_line[middle_idx];
            }
        }

        res
    }

    #[test]
    fn test_example() {
        let input = indoc!(
            r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "#
        );

        assert_eq!(143, solution(input));
    }

    #[test]
    fn test_input() {
        let input = std::fs::read_to_string("input.txt").unwrap();

        assert_eq!(5_955, solution(&input));
    }
}
