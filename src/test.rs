#[test]
fn test_graph() {
    let input = b"3 4\n1 2\n1 3\n2 3\n2 1";
    let reader = Cursor::new(input);

    input! {
        reader =>
            n, m: usize,
            edges: [(usize1, usize1), m]
    }

    assert_eq!(n, 3);
    assert_eq!(m, 4);
    assert_eq!(edges, vec![(0, 1), (0, 2), (1, 2), (1, 0)]);
}
