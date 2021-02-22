use super::*;
use linked_list::LinkedList;
use min_stack::MinStack;
use crate::my_queue::MyQueue;

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

#[test]
fn search_range_test() {
    let test_cases = vec![
        (vec![], 1, vec![-1, -1]),
        (vec![1], 1, vec![0, 0]),
        (vec![1, 1], 1, vec![0, 1]),
        (vec![1, 2, 3, 3, 5], 3, vec![2, 3]),
    ];
    for case in test_cases {
        assert_eq!(search_range(case.0, case.1), case.2);
    }
}

fn get_reverse_list_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
    vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![1, 2, 3], vec![3, 2, 1]),
    ]
}

#[test]
fn reverse_list_test() {
    for case in get_reverse_list_test_cases() {
        assert_eq!(reverse_list(ListNode::from_slice(&case.0)).to_vec(), case.1);
    }
}

#[test]
fn min_stack_test() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}

#[test]
fn my_queue_test() {
    let mut obj = MyQueue::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.pop(), -2);
    assert_eq!(obj.peek(), 0);
    assert_eq!(obj.empty(), false);
}

#[test]
fn remove_duplicates_test() {
    let test_cases = vec![
        ("aaaabcdeeef", 2, "bcdef"),
        ("abcd", 2, "abcd"),
        ("deeedbbcccbdaa", 3, "aa"),
        ("pbbcggttciiippooaais", 2, "ps"),
    ];
    for case in test_cases {
        assert_eq!(remove_duplicates(case.0.to_string(), case.1), case.2.to_string());
    }
}

#[test]
fn quick_sort_test() {
    let test_cases = vec![
        vec![],
        vec![1],
        vec![1, 1],
        vec![1, 2],
        vec![2, 1],
        vec![11, 18, 1, 20, 5, 12, 5, 16, 21, 9, 13, 17, 3, 24, 15, 19],
        vec![11, 11, 18, 1, 20, 5, 12, 5, 16, 21, 9, 13, 17, 3, 24, 15, 19, 11],
    ];
    for mut case in test_cases {
        let mut expected = case.clone();
        expected.sort_unstable();
        quick_sort(&mut case);
        assert_eq!(case, expected);
    }
}

#[test]
fn matrix_dfs_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![vec![1]], vec![1]),
        (vec![vec![1, 2], vec![3, 4]], vec![1, 2, 4, 3]),
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![1, 2, 3, 6, 9, 8, 7, 4, 5]),
    ];
    for case in test_cases {
        assert_eq!(matrix_dfs(case.0), case.1);
    }
}

#[test]
fn matrix_bfs_test() {
    let test_cases = vec![
        (vec![], vec![]),
        (vec![vec![1]], vec![1]),
        (vec![vec![1, 2], vec![3, 4]], vec![1, 2, 3, 4]),
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], vec![1, 2, 4, 3, 5, 7, 6, 8, 9]),
    ];
    for case in test_cases {
        assert_eq!(matrix_bfs(case.0), case.1);
    }
}