use crate::arrays::Solution;

#[test]
fn two_sum_test() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![1, 0]),
        (vec![3, 2, 4], 6, vec![2, 1]),
        (vec![3, 3], 6, vec![1, 0]),
    ];
    for case in test_cases {
        assert_eq!(Solution::two_sum(case.0, case.1), case.2)
    }
}

#[test]
fn is_palindrome_test() {
    let test_cases = vec![
        (121, true),
        (123, false),
        (1221, true),
        (1231, false),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_palindrome(case.0), case.1)
    }
}

#[test]
fn is_valid_test() {
    let test_cases = vec![
        ("(){}", true),
        (")(", false),
        ("(){", false),
        ("[{()}]", true),
        ("((", false),
        ("(){}}{", false),
    ];
    for case in test_cases {
        assert_eq!(Solution::is_valid(case.0.to_string()), case.1);
    }
}

#[test]
fn longest_common_prefix_test() {
    let test_cases = vec![
        (vec!["flower".to_string(), "flow".to_string(), "flight".to_string()], "fl"),
        (vec!["flower".to_string(), "flow".to_string(), "".to_string()], ""),
        (vec!["abc".to_string(), "abc".to_string(), "abc".to_string()], "abc"),
    ];
    for case in test_cases {
        assert_eq!(Solution::longest_common_prefix(case.0), case.1);
    }
}

#[test]
fn remove_element_test() {
    let test_cases = vec![
        (vec![3, 2, 2, 3], 3, 2),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5),
        (vec![0, 1, 3, 1, 3, 0, 4, 1], 2, 8),
        (vec![], 2, 0),
        (vec![1], 2, 1),
        (vec![1], 1, 0),
    ];
    for case in test_cases {
        let mut nums = case.0;
        assert_eq!(Solution::remove_element(&mut nums, case.1), case.2);
    }
}