// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, item) in nums.iter().enumerate() {
            let res = target - item;
            for (v, vitem)  in nums.iter().enumerate() {
                let ret = res - vitem;
                if ret == 0 && i != v {
                    return vec![i as i32, v as i32];
                }
            }
        }
        vec![]
    }
}