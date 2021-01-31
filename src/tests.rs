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
        assert_eq!(binary_search(case.0, case.1), case.2);
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
        assert_eq!(pivot_index(case.0), case.1);
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

#[test]
fn max_sub_array_test() {
    let test_cases = vec![
        (vec![1, 2, 3, 4], 10),
        (vec![1, -2, 3, -4], 3),
        (vec![10, -2, 3, -4], 11),
    ];
    for case in test_cases {
        assert_eq!(max_sub_array(case.0), case.1);
    }
}

#[test]
fn max_profit_test() {
    let test_cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
    ];
    for case in test_cases {
        assert_eq!(max_profit(case.0), case.1);
    }
}

fn get_binary_search_rotated_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
    vec![
        (vec![], 1, -1),
        (vec![1], 1, 0),
        (vec![5, 6, 0, 1, 2, 3, 4], 6, 1),
        (vec![5, 6, 0, 1, 2, 3, 4], 2, 4),
        (vec![5, 6, 0, 1, 2, 3, 4], 0, 2),
        (vec![1, 3], 1, 0),
        (vec![1, 3], 3, 1),
        (vec![3, 1], 3, 0),
        (vec![3, 1], 1, 1),
    ]
}

#[test]
fn binary_search_rotated_test() {
    for case in get_binary_search_rotated_test_cases() {
        assert_eq!(binary_search_rotated(case.0, case.1), case.2);
    }
}

#[test]
fn binary_search_rotated_v2_test() {
    for case in get_binary_search_rotated_test_cases() {
        assert_eq!(binary_search_rotated_v2(case.0, case.1), case.2);
    }
}

fn get_sorted_squares_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![], vec![]),
        (vec![-1], vec![1]),
        (vec![-2, 0], vec![0, 4]),
        (vec![1, 2], vec![1, 4]),
        (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
        (vec![-7, -3, 2, 3, 11], vec![4, 9, 9, 49, 121]),
    ]
}

#[test]
fn sorted_squares_test() {
    for case in get_sorted_squares_test_cases() {
        assert_eq!(sorted_squares(case.0), case.1);
    }
}