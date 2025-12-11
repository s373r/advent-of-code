// \--- Day 11: Reactor ---
// ----------
//
// You hear some loud beeping coming from a hatch in the floor of the factory, so you decide to check it out. Inside, you find several large electrical conduits and a ladder.
//
// Climbing down the ladder, you discover the source of the beeping: a large, toroidal reactor which powers the factory above. Some Elves here are hurriedly running between the reactor and a nearby server rack, apparently trying to fix something.
//
// One of the Elves notices you and rushes over. "It's a good thing you're here! We just installed a new *server rack*, but we aren't having any luck getting the reactor to communicate with it!" You glance around the room and see a tangle of cables and devices running from the server rack to the reactor. She rushes off, returning a moment later with a list of the devices and their outputs (your puzzle input).
//
// For example:
//
// ```
// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out
//
// ```
//
// Each line gives the name of a device followed by a list of the devices to which its outputs are attached. So, `bbb: ddd eee` means that device `bbb` has two outputs, one leading to device `ddd` and the other leading to device `eee`.
//
// The Elves are pretty sure that the issue isn't due to any specific device, but rather that the issue is triggered by data following some specific *path* through the devices. Data only ever flows from a device through its outputs; it can't flow backwards.
//
// After dividing up the work, the Elves would like you to focus on the devices starting with the one next to you (an Elf hastily attaches a label which just says *`you`*) and ending with the main output to the reactor (which is the device with the label *`out`*).
//
// To help the Elves figure out which path is causing the issue, they need you to find *every* path from `you` to `out`.
//
// In this example, these are all of the paths from `you` to `out`:
//
// * Data could take the connection from `you` to `bbb`, then from `bbb` to `ddd`, then from `ddd` to `ggg`, then from `ggg` to `out`.
// * Data could take the connection to `bbb`, then to `eee`, then to `out`.
// * Data could go to `ccc`, then `ddd`, then `ggg`, then `out`.
// * Data could go to `ccc`, then `eee`, then `out`.
// * Data could go to `ccc`, then `fff`, then `out`.
//
// In total, there are `*5*` different paths leading from `you` to `out`.
//
// *How many different paths lead from `you` to `out`?*

use std::collections::HashMap;
use std::str::SplitWhitespace;

fn solution(input: &str) -> usize {
    const START_LABEL: &str = "you";
    const END_LABEL: &str = "out";

    fn path_traveling(labels_mapping: &HashMap<&str, SplitWhitespace>, label: &str) -> usize {
        let outs = labels_mapping.get(label).unwrap();

        let mut path_count = 0;

        for out in outs.clone() {
            if out == END_LABEL {
                return 1;
            }

            path_count += path_traveling(labels_mapping, out);
        }

        path_count
    }

    let labels_mapping = input
        .lines()
        .map(|line| {
            let mut line_chunk_iter = line.split(":");

            let label = line_chunk_iter.next().unwrap();
            let outputs = {
                let rest_line = line_chunk_iter.next().unwrap();
                rest_line.split_whitespace()
            };

            (label, outputs)
        })
        .collect::<HashMap<_, _>>();

    path_traveling(&labels_mapping, START_LABEL)
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        aaa: you hhh
        you: bbb ccc
        bbb: ddd eee
        ccc: ddd eee fff
        ddd: ggg
        eee: out
        fff: out
        ggg: out
        hhh: ccc fff iii
        iii: out
        "#
    );

    assert_eq!(5, solution(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(636, solution(&input));
}
