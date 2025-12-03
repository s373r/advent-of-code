fn solution(_input: &str) -> usize {
    todo!()
}

#[test]
fn test_example() {
    let input = indoc::indoc!(
        r#"
        TODO
        "#
    );

    assert_eq!(42, solution(input));
}

#[test]
fn test_input() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    assert_eq!(42, solution(&input));
}
