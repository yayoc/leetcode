// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].
pub struct Solution1;

use std::collections::HashMap;

pub trait Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>;
}

impl Solution for Solution1 {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if hash.contains_key(num) {
                return vec![hash[num] as i32, i as i32];
            } else {
                hash.insert(target - num, i);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution1::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution1::two_sum(vec![3, 2, 4], 6));
    }
}