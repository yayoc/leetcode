// Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.

// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

pub struct Solution1;

pub trait Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32;
}

impl Solution for Solution1 {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_valid() {
        let mut t1: Vec<i32> = vec![1, 1, 2];
        assert_eq!(2, Solution1::remove_duplicates(&mut t1));
        let mut t2: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, Solution1::remove_duplicates(&mut t2));
    }
}
