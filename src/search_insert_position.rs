// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2

pub struct Solution1;

pub trait Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

impl Solution for Solution1 {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.iter().position(|&n| n >= target) {
            Some(x) => x as i32,
            None => nums.len() as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_str() {
        assert_eq!(2, Solution1::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution1::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution1::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution1::search_insert(vec![1, 3, 5, 6], 0));
    }
}
