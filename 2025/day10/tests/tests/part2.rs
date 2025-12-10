// \--- Part Two ---
// ----------
//
// All of the machines are starting to come online! Now, it's time to worry about the joltage requirements.
//
// Each machine needs to be configured to *exactly the specified joltage levels* to function properly. Below the buttons on each machine is a big lever that you can use to switch the buttons from configuring the indicator lights to increasing the joltage levels. (Ignore the indicator light diagrams.)
//
// The machines each have a set of *numeric counters* tracking its joltage levels, one counter per joltage requirement. The counters are all *initially set to zero*.
//
// So, joltage requirements like `{3,5,4,7}` mean that the machine has four counters which are initially `0` and that the goal is to simultaneously configure the first counter to be `3`, the second counter to be `5`, the third to be `4`, and the fourth to be `7`.
//
// The button wiring schematics are still relevant: in this new joltage configuration mode, each button now indicates which counters it affects, where `0` means the first counter, `1` means the second counter, and so on. When you push a button, each listed counter is *increased by `1`*.
//
// So, a button wiring schematic like `(1,3)` means that each time you push that button, the second and fourth counters would each increase by `1`. If the current joltage levels were `{0,1,2,3}`, pushing the button would change them to be `{0,2,2,4}`.
//
// You can push each button as many times as you like. However, your finger is getting sore from all the button pushing, and so you will need to determine the *fewest total presses* required to correctly configure each machine's joltage level counters to match the specified joltage requirements.
//
// Consider again the example from before:
//
// ```
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
//
// ```
//
// Configuring the first machine's counters requires a minimum of `*10*` button presses. One way to do this is by pressing `(3)` once, `(1,3)` three times, `(2,3)` three times, `(0,2)` once, and `(0,1)` twice.
//
// Configuring the second machine's counters requires a minimum of `*12*` button presses. One way to do this is by pressing `(0,2,3,4)` twice, `(2,3)` five times, and `(0,1,2)` five times.
//
// Configuring the third machine's counters requires a minimum of `*11*` button presses. One way to do this is by pressing `(0,1,2,3,4)` five times, `(0,1,2,4,5)` five times, and `(1,2)` once.
//
// So, the fewest button presses required to correctly configure the joltage level counters on all of the machines is `10` + `12` + `11` = `*33*`.
//
// Analyze each machine's joltage requirements and button wiring schematics. *What is the fewest button presses required to correctly configure the joltage level counters on all of the machines?*

fn nested_loop(
    depth: usize,
    max: usize,
    indices_store: &mut Vec<usize>,
    f: &mut impl FnMut(&[usize]) -> bool,
) {
    // [0]
    // [0, 0]
    // [0, 0, 0]
    // [0, 0, 1]
    // [0, 0, 2]
    // [0, 1]
    // [0, 1, 0]
    // [0, 1, 1]
    // [0, 1, 2]
    // [0, 2]
    // [0, 2, 0]
    // ...
    for i in 0..max {
        indices_store.push(i);
        let continue_recursion = f(indices_store) && indices_store.len() < depth;

        if continue_recursion {
            nested_loop(depth, max, indices_store, f);
        }

        indices_store.pop();
    }
}

fn solution(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let mut line_chunk_iter = line.split_whitespace();

        let _ignored_indicator_light_diagram = line_chunk_iter.next().unwrap();
        let joltage_requirements = line_chunk_iter
            .next_back()
            .unwrap()
            .trim_matches(|c| matches!(c, '{' | '}'))
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        let button_wiring_schematics = line_chunk_iter
            .map(|b| {
                b.trim_matches(|c| matches!(c, '(' | ')'))
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<usize>, _>>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        // Increase if the answer is not suitable
        const MAX_TOTAL_PRESSES: usize = 20;

        let mut fewest_total_presses = usize::MAX;
        let mut indices_store = Vec::with_capacity(MAX_TOTAL_PRESSES);
        let mut joltage_levels = vec![0; joltage_requirements.len()];

        nested_loop(
            MAX_TOTAL_PRESSES,
            button_wiring_schematics.len(),
            &mut indices_store,
            &mut |indices| {
                let button_presses = indices.len();

                if button_presses > fewest_total_presses {
                    // There is no point in performing further recursive calculations of this direction,
                    // since we already have a better option.
                    return false;
                }

                // The counters are all initially set to zero (c)
                joltage_levels.fill(0);

                // Press buttons
                for idx in indices {
                    for button in &button_wiring_schematics[*idx] {
                        joltage_levels[*button] += 1;
                    }
                }

                // {
                //     /* debug */
                //     println!("             indices: {:?}", indices);
                //     println!("      joltage levels: {:?}", joltage_levels);
                //     println!("joltage_requirements: {:?}", joltage_requirements);
                // }

                // Is that acceptable?
                let mut is_equal = true;
                for (actual, expected) in joltage_levels.iter().zip(joltage_requirements.iter()) {
                    use std::cmp::Ordering;

                    match actual.cmp(expected) {
                        Ordering::Less => {
                            is_equal = false;
                        }
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            // We've already moved on to one of the counters, there's no point in continuing.
                            return false;
                        }
                    }
                }

                if is_equal {
                    if fewest_total_presses > button_presses {
                        fewest_total_presses = button_presses;
                    }
                    // There is no point in continuing this direction of calculation
                    // if we have already found the combination.
                    false
                } else {
                    true
                }
            },
        );

        acc + fewest_total_presses
    })
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "#
    );

    assert_eq!(33, solution(input));
}

#[test]
fn test_input() {
    todo!("not solved");
    // let input = std::fs::read_to_string("input.txt").unwrap();
    //
    // assert_eq!(42, solution(&input));
}
