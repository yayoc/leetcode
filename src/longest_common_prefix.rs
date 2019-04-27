// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
// Example 1:
//
// Input: ["flower","flow","flight"]
// Output: "fl"
//
// Example 2:
//
// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
//
// Note:
//
// All given inputs are in lowercase letters a-z.

pub struct Solution1;

pub trait Solution {
    fn longest_common_prefix(strs: Vec<String>) -> String;
}

impl Solution for Solution1 {
    fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();
        for i in 0..min_len {
            let ch = strs[0].chars().nth(i).unwrap();

            for s in &strs {
                if s.chars().nth(i).unwrap() != ch {
                    return s[0..i].to_string();
                }
            }
        }
        strs[0][0..min_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            "fl",
            Solution1::longest_common_prefix(vec!(
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ))
        );
        assert_eq!(
            "",
            Solution1::longest_common_prefix(vec!(
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ))
        );
        assert_eq!(
            "",
            Solution1::longest_common_prefix(vec!(String::from("aca"), String::from("cba")))
        )
    }
}
