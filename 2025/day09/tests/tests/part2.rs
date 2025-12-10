// \--- Part Two ---
// ----------
//
// The Elves just remembered: they can only switch out tiles that are *red* or *green*. So, your rectangle can only include red or green tiles.
//
// In your list, every red tile is connected to the red tile before and after it by a straight line of *green tiles*. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.
//
// Using the same example as before, the tiles marked `X` would be green:
//
// ```
// ..............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............
//
// ```
//
// In addition, all of the tiles *inside* this loop of red and green tiles are *also* green. So, in this example, these are the green tiles:
//
// ```
// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............
//
// ```
//
// The remaining tiles are never red nor green.
//
// The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.
//
// For example, you could make a rectangle out of red and green tiles with an area of `15` between `7,3` and `11,1`:
//
// ```
// ..............
// .......OOOOO..
// .......OOOOO..
// ..#XXXXOOOOO..
// ..XXXXXXXXXX..
// ..#XXXXXX#XX..
// .........XXX..
// .........#X#..
// ..............
//
// ```
//
// Or, you could make a thin rectangle with an area of `3` between `9,7` and `9,5`:
//
// ```
// ..............
// .......#XXX#..
// .......XXXXX..
// ..#XXXX#XXXX..
// ..XXXXXXXXXX..
// ..#XXXXXXOXX..
// .........OXX..
// .........OX#..
// ..............
//
// ```
//
// The largest rectangle you can make in this example using only red and green tiles has area `*24*`. One way to do this is between `9,5` and `2,3`:
//
// ```
// ..............
// .......#XXX#..
// .......XXXXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// ..OOOOOOOOXX..
// .........XXX..
// .........#X#..
// ..............
//
// ```
//
// Using two red tiles as opposite corners, *what is the largest area of any rectangle you can make using only red and green tiles?*

// WARNING:
// An absolutely terrible implementation that requires... 32-38 GB of RAM and a significant amount of time to complete.
fn solution(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|line| {
            let mut coord_iter = line.split(",").map(str::parse::<usize>).map(Result::unwrap);
            let x = coord_iter.next().unwrap();
            let y = coord_iter.next().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let ((min_x, min_y), (max_x, max_y)) = coords.iter().fold(
        ((usize::MAX, usize::MAX), (0, 0)),
        |((min_x, min_y), (max_x, max_y)), (x, y)| {
            (
                (min_x.min(*x), min_y.min(*y)),
                (max_x.max(*x), max_y.max(*y)),
            )
        },
    );

    let relative_max_x = max_x - min_x + 1;
    let relative_max_y = max_y - min_y + 1;

    let mut m = (0..relative_max_y)
        .map(|_| vec!['.'; relative_max_x])
        .collect::<Vec<_>>();

    let offset_x = min_x;
    let offset_y = min_y;

    let mut coord_iter = coords.iter();
    let prev_p = coord_iter.next().unwrap();
    let (first_x, first_y) = (prev_p.0, prev_p.1);
    let (mut prev_x, mut prev_y) = (first_x, first_y);
    for (x, y) in coord_iter.chain(std::iter::once(&(first_x, first_y))) {
        let min_x = prev_x.min(*x);
        let min_y = prev_y.min(*y);
        let max_x = prev_x.max(*x);
        let max_y = prev_y.max(*y);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                m[y - offset_y][x - offset_x] = 'X';
            }
        }

        // {
        //     /* debug */
        //     m[prev_y - offset_y][prev_x - offset_x] = '#';
        //     m[y - offset_y][x - offset_x] = '#';
        //
        //     for line in &m {
        //         println!("{}", line.iter().collect::<String>());
        //     }
        //     println!();
        // }

        prev_x = *x;
        prev_y = *y;
    }

    // {
    //     /* debug */
    //     for line in &m {
    //         println!("{}", line.iter().collect::<String>());
    //     }
    //     println!();
    // }

    {
        // from left to right
        for y in 0..relative_max_y {
            for x in 0..relative_max_x {
                match m[y][x] {
                    '.' => m[y][x] = '*',
                    _ => break,
                }
            }
        }
        // from top to bottom
        for x in 0..relative_max_x {
            for y in 0..relative_max_y {
                match m[y][x] {
                    '.' => m[y][x] = '*',
                    _ => break,
                }
            }
        }
        // from right to left
        for y in 0..relative_max_y {
            for x in relative_max_x..0 {
                match m[y][x] {
                    '.' => m[y][x] = '*',
                    _ => break,
                }
            }
        }
        // from bottom to top
        for x in 0..relative_max_x {
            for y in relative_max_y..0 {
                match m[y][x] {
                    '.' => m[y][x] = '*',
                    _ => break,
                }
            }
        }
    }
    // {
    //     /* debug */
    //     for line in &m {
    //         println!("{}", line.iter().collect::<String>());
    //     }
    //     println!();
    // }

    let mut max_area = 0;

    for p1 @ (x1, y1) in &coords {
        'p2_loop: for p2 @ (x2, y2) in &coords {
            if p1 == p2 {
                continue;
            }

            let min_x = *x1.min(x2);
            let min_y = *y1.min(y2);
            let max_x = *x1.max(x2);
            let max_y = *y1.max(y2);

            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    match m[y - offset_y][x - offset_x] {
                        '#' | 'X' | '.' => { /* Ok */ }
                        _ => continue 'p2_loop,
                    }
                }
            }

            let area = x1.abs_diff(x2 + 1) * y1.abs_diff(y2 + 1);

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
        "#
    );

    assert_eq!(24, solution(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(4761736832, solution(&input));
}
