// Given an array nums and a value val, remove all instances of that value in-place and return the new length.
//
// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
//
// The order of elements can be changed. It doesn't matter what you leave beyond the new length.

pub struct Solution1;

pub trait Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32;
}

impl Solution for Solution1 {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&n| n != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_element() {
        assert_eq!(2, Solution1::remove_element(vec![3, 2, 2, 3].as_mut(), 3));
        assert_eq!(5, Solution1::remove_element(vec![0,1,2,2,3,0,4,2].as_mut(), 2));
    }
}
