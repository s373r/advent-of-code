// \--- Part Two ---
// ----------
//
// Thanks in part to your analysis, the Elves have figured out a little bit about the issue. They now know that the problematic data path passes through both `dac` (a [digital-to-analog converter](https://en.wikipedia.org/wiki/Digital-to-analog_converter)) and `fft` (a device which performs a [fast Fourier transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform)).
//
// They're still not sure which specific path is the problem, and so they now need you to find every path from `svr` (the server rack) to `out`. However, the paths you find must all also visit both `dac` *and* `fft` (in any order).
//
// For example:
//
// ```
// svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out
//
// ```
//
// This new list of devices contains many paths from `svr` to `out`:
//
// ```
// svr,aaa,fft,ccc,ddd,hub,fff,ggg,out
// svr,aaa,fft,ccc,ddd,hub,fff,hhh,out
// svr,aaa,fft,ccc,eee,dac,fff,ggg,out
// svr,aaa,fft,ccc,eee,dac,fff,hhh,out
// svr,bbb,tty,ccc,ddd,hub,fff,ggg,out
// svr,bbb,tty,ccc,ddd,hub,fff,hhh,out
// svr,bbb,tty,ccc,eee,dac,fff,ggg,out
// svr,bbb,tty,ccc,eee,dac,fff,hhh,out
//
// ```
//
// However, only *`2`* paths from `svr` to `out` visit both `dac` and `fft`.
//
// Find all of the paths that lead from `svr` to `out`. *How many of those paths visit both `dac` and `fft`?*

use std::collections::HashMap;
use std::str::SplitWhitespace;

fn solution(input: &str) -> usize {
    const START_LABEL: &str = "svr";
    const DAC_LABEL: &str = "dac";
    const FFT_LABEL: &str = "fft";
    const END_LABEL: &str = "out";

    const MAX_DEPTH: usize = 18;

    fn path_traveling(
        labels_mapping: &mut HashMap<&str, SplitWhitespace>,
        // TODO: looks like we need some kind of cache
        labels_cache: &mut HashMap<&str, usize>,
        label: &str,
        // TODO: use?
        start_label: &str,
        mut dac_seen: bool,
        mut fft_seen: bool,
        depth: usize,
    ) -> usize {
        if depth == MAX_DEPTH {
            return 0;
        }

        match label {
            DAC_LABEL => {
                dac_seen = true;
            }
            FFT_LABEL => {
                fft_seen = true;
            }
            _ => {}
        }

        let outs = labels_mapping.get(label).unwrap();

        let mut path_count = 0;

        for out in outs.clone() {
            if out == END_LABEL {
                return if dac_seen && fft_seen { 1 } else { 0 };
            }

            path_count += path_traveling(
                labels_mapping,
                labels_cache,
                out,
                start_label,
                dac_seen,
                fft_seen,
                depth + 1,
            );
        }

        path_count
    }

    let mut labels_mapping = input
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
    let mut labels_cache = HashMap::new();

    path_traveling(
        &mut labels_mapping,
        &mut labels_cache,
        START_LABEL,
        START_LABEL,
        false,
        false,
        0,
    )
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out
        "#
    );

    assert_eq!(2, solution(input));
}

#[test]
fn test_input() {
    // let input = std::fs::read_to_string("input.txt").unwrap();
    //
    // assert_eq!(636, solution(&input));
    todo!("not solved");
}
