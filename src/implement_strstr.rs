// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "hello", needle = "ll"
// Output: 2

// Example 2:

// Input: haystack = "aaaaa", needle = "bba"
// Output: -1

pub struct Solution1;

pub trait Solution {
    fn str_str(haystack: String, needle: String) -> i32;
}

impl Solution for Solution1 {
    fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }

        match haystack.find(&needle) {
            Some(i) => i as i32,
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_str_str() {
        assert_eq!(
            2,
            Solution1::str_str(String::from("hello"), String::from("ll"))
        );
    }
}
