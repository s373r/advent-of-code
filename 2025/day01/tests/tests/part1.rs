fn calculate_position(
    position: u64,
    increment: i32,
    range_start: i32,
    range_end_inclusive: i32,
) -> u64 {
    let range_length = (range_end_inclusive + 1) - range_start;
    let new_position = position as i32 + increment;

    (range_start + (new_position - range_start).rem_euclid(range_length)) as u64
}

fn decode_password(input: &str) -> u64 {
    let lines = input.lines();

    let mut dual_position = 50;
    let mut password = 0;

    for line in lines {
        let delta = match (line.as_bytes().first(), line[1..].parse::<i32>()) {
            (Some(b'L'), Ok(delta)) => -delta,
            (Some(b'R'), Ok(delta)) => delta,
            _ => unreachable!("Unexpected line: {line}"),
        };

        dual_position = calculate_position(dual_position, delta, 0, 99);

        if dual_position == 0 {
            password += 1;
        }
    }

    password
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        "#
    );

    assert_eq!(3, decode_password(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(1154, decode_password(&input));
}
