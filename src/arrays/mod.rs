use std::collections::HashMap;
mod tests;

struct Solution;
impl Solution {

    // https://leetcode.com/problems/two-sum/description/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut buffer = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            match buffer.get(&num) {
                Some(&j) => return vec![i as i32, j],
                None => {
                    buffer.insert(target - num, i as i32);
                }
            }
        }
        unreachable!()
    }
}