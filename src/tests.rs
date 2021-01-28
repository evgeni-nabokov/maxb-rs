use super::*;

#[test]
fn binary_search_test() {
    let test_cases = vec![
        (vec![], 1, -1),
        (vec![1, 20, 32, 39, 40, 45, 68, 77, 93], 1, 0),
        (vec![1, 20, 32, 39, 40, 45, 68, 77, 93], 100, -1),
        (vec![1, 20, 32, 39, 40, 45, 68, 77, 93], 93, 8),
        (vec![1, 20, 32, 39, 40, 45, 68, 77, 93], 40, 4),
    ];
    for case in test_cases {
        assert_eq!(binary_search(&case.0, case.1), case.2);
    }
}

#[test]
fn shuffle_test() {
    let test_cases = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    ];
    for mut case in test_cases {
        let original = case.clone();
        shuffle(&mut case);
        assert_ne!(case, original);
    }
}

#[test]
fn pivot_index_test() {
    let test_cases = vec![
        (vec![], -1),
        (vec![1, 2], -1),
        (vec![-1, 2, -1], 1),
        (vec![-1, 5, -4, 8], -1),
        (vec![-1, 5, 2, -4, 8], 2),
    ];
    for case in test_cases {
        assert_eq!(pivot_index(&case.0), case.1);
    }
}

#[test]
fn rotate_test() {
    let test_cases = vec![
        (vec![], 1, vec![]),
        (vec![1, 2, 3], 0, vec![1, 2, 3]),
        (vec![1, 2, 3], 6, vec![1, 2, 3]),
        (vec![1, 2, 3, 4, 5], 1, vec![5, 1, 2, 3, 4]),
        (vec![1, 2, 3, 4, 5], 4, vec![2, 3, 4, 5, 1]),
    ];
    for mut case in test_cases {
        rotate(&mut case.0, case.1);
        assert_eq!(case.0, case.2);
    }
}