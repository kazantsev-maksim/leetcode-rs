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