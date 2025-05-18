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

    // https://leetcode.com/problems/palindrome-number/
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }

    // https://leetcode.com/problems/valid-parentheses/
    pub fn is_valid(s: String) -> bool {
        let mut brackets = Vec::with_capacity(s.len());
        for bracket in s.chars() {
            match bracket {
                '(' => brackets.push(')'),
                '{' => brackets.push('}'),
                '[' => brackets.push(']'),

                closing => if Some(closing) != brackets.pop() {
                    return false
                }
            }
        }
        brackets.is_empty()
    }

    // https://leetcode.com/problems/longest-common-prefix/
    pub fn longest_common_prefix(input: Vec<String>) -> String {
        input.into_iter().reduce(|prefix, next|{
            prefix.chars()
                .zip(next.chars())
                .take_while(|(a,c)| a == c)
                .map(|(c,_)|c)
                .collect()
        }).unwrap()
    }

    // https://leetcode.com/problems/remove-element/
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        idx as i32
    }
}